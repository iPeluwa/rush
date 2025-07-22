use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::process::Stdio;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use tokio::process::Command;

use crate::cache::TaskCache;
use crate::graph::{Task, TaskGraph};

pub struct TaskExecutor {
    graph: TaskGraph,
    cache: TaskCache,
    verbose: bool,
}

impl TaskExecutor {
    pub fn new(graph: TaskGraph, verbose: bool) -> Self {
        Self {
            graph,
            cache: TaskCache::new(),
            verbose,
        }
    }

    pub async fn execute_task(&self, task_name: &str) -> Result<()> {
        // Validate task exists
        if !self.graph.tasks.contains_key(task_name) {
            return Err(anyhow::anyhow!(
                "❌ Task '{}' not found. Available tasks: {}",
                task_name,
                self.graph.tasks.keys().cloned().collect::<Vec<_>>().join(", ")
            ));
        }

        let execution_order = self.graph.topological_sort(task_name)
            .with_context(|| format!("Failed to resolve dependencies for task '{}'", task_name))?;

        if self.verbose {
            println!("🔍 Debug: Task dependency resolution");
            println!("   Target task: {}", task_name);
            println!("   Execution order: {}", execution_order.join(" -> "));
            println!("   Total tasks to run: {}", execution_order.len());
        } else {
            println!("Execution order: {}", execution_order.join(" -> "));
        }

        for task_name in execution_order {
            if let Some(task) = self.graph.tasks.get(&task_name) {
                if self.verbose {
                    println!("🔍 Debug: About to execute task '{}'", task.name);
                    println!("   Command: {}", task.cmd);
                    if !task.env.is_empty() {
                        println!("   Environment: {:?}", task.env);
                    }
                }
                self.run_single_task(task).await
                    .with_context(|| format!("Task '{}' failed during execution", task.name))?;
            }
        }

        Ok(())
    }

    pub async fn execute_task_parallel(&self, task_name: &str) -> Result<()> {
        let execution_order = self.graph.topological_sort(task_name)?;

        println!(
            "🚀 Parallel execution order: {}",
            execution_order.join(" -> ")
        );

        // Group tasks by dependency level for parallel execution
        let levels = self.build_execution_levels(&execution_order)?;

        for (level, tasks) in levels.iter().enumerate() {
            if tasks.len() == 1 {
                // Single task - run normally  
                if let Some(task) = self.graph.tasks.get(&tasks[0]) {
                    let task_progress = ProgressBar::new_spinner();
                    task_progress.set_style(
                        ProgressStyle::default_spinner()
                            .template("🏃 {msg} {spinner:.green}")
                            .unwrap(),
                    );
                    task_progress.set_message(format!("Running {}", task.name));
                    
                    self.run_single_task_with_progress(task, &task_progress).await?;
                    task_progress.finish_with_message(format!("✅ {} completed", task.name));
                }
            } else {
                // Multiple independent tasks - run in parallel
                println!(
                    "📦 Level {}: Running {} tasks in parallel",
                    level,
                    tasks.len()
                );

                let mut handles = Vec::new();

                for task_name in tasks {
                    if let Some(task) = self.graph.tasks.get(task_name) {
                        let task = task.clone();
                        let cache = TaskCache::new();
                        let task_progress = ProgressBar::new_spinner();
                        task_progress.set_style(
                            ProgressStyle::default_spinner()
                                .template("🏃 {msg} {spinner:.green}")
                                .unwrap(),
                        );
                        task_progress.set_message(format!("Running {}", task.name));

                        let handle = tokio::spawn(async move {
                            let result = Self::run_task_standalone_with_progress(&task, &cache, &task_progress).await;
                            if result.is_ok() {
                                task_progress.finish_with_message(format!("✅ {} completed", task.name));
                            } else {
                                task_progress.finish_with_message(format!("❌ {} failed", task.name));
                            }
                            result
                        });

                        handles.push((task_name.clone(), handle));
                    }
                }

                // Wait for all parallel tasks to complete
                for (task_name, handle) in handles {
                    match handle.await {
                        Ok(Ok(())) => {
                            // Task completed successfully - already logged in run_task_standalone
                        }
                        Ok(Err(e)) => {
                            return Err(anyhow::anyhow!("Task '{}' failed: {}", task_name, e));
                        }
                        Err(e) => {
                            return Err(anyhow::anyhow!("Task '{}' panicked: {}", task_name, e));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn build_execution_levels(&self, execution_order: &[String]) -> Result<Vec<Vec<String>>> {
        let mut levels = Vec::new();
        let mut completed = std::collections::HashSet::new();
        let mut remaining: Vec<String> = execution_order.to_vec();

        while !remaining.is_empty() {
            let mut ready_tasks = Vec::new();

            // Find tasks that can run now (all dependencies completed)
            for task_name in &remaining {
                if let Some(deps) = self.graph.dependencies.get(task_name) {
                    if deps.iter().all(|dep| completed.contains(dep)) {
                        ready_tasks.push(task_name.clone());
                    }
                } else {
                    // No dependencies
                    ready_tasks.push(task_name.clone());
                }
            }

            if ready_tasks.is_empty() {
                return Err(anyhow::anyhow!(
                    "Dependency cycle detected or invalid state"
                ));
            }

            // Mark ready tasks as completed and remove from remaining
            for task in &ready_tasks {
                completed.insert(task.clone());
            }
            remaining.retain(|task| !ready_tasks.contains(task));

            levels.push(ready_tasks);
        }

        Ok(levels)
    }

    async fn run_task_standalone(task: &Task, cache: &TaskCache) -> Result<()> {
        // Check cache if cache files are specified
        if !task.cache_files.is_empty() {
            let hash = cache.compute_task_hash(&task.name, &task.cache_files)?;
            if cache.is_cached(&task.name, &hash) {
                println!("⚡ Task '{}' skipped (cached)", task.name);
                return Ok(());
            }
        }

        println!("🏃 Running task: {}", task.name);

        let mut cmd = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", &task.cmd]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", &task.cmd]);
            cmd
        };

        // Set environment variables
        for (key, value) in &task.env {
            cmd.env(key, value);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd.output().await?;

        if output.status.success() {
            println!("✅ Task '{}' completed successfully", task.name);
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }

            // Cache the result if cache files are specified
            if !task.cache_files.is_empty() {
                let hash = cache.compute_task_hash(&task.name, &task.cache_files)?;
                cache.mark_cached(&task.name, &hash)?;
            }
        } else {
            println!("❌ Task '{}' failed", task.name);
            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
            anyhow::bail!(
                "Task '{}' failed with exit code: {:?}",
                task.name,
                output.status.code()
            );
        }

        Ok(())
    }

    async fn run_single_task(&self, task: &Task) -> Result<()> {
        // Check cache if cache files are specified
        if !task.cache_files.is_empty() {
            let hash = self
                .cache
                .compute_task_hash(&task.name, &task.cache_files)?;
            if self.cache.is_cached(&task.name, &hash) {
                println!("⚡ Task '{}' skipped (cached)", task.name);
                return Ok(());
            }
        }

        println!("🏃 Running task: {}", task.name);

        let mut cmd = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", &task.cmd]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", &task.cmd]);
            cmd
        };

        // Set environment variables
        for (key, value) in &task.env {
            cmd.env(key, value);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd.output().await?;

        if output.status.success() {
            println!("✅ Task '{}' completed successfully", task.name);
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }

            // Cache the result if cache files are specified
            if !task.cache_files.is_empty() {
                let hash = self
                    .cache
                    .compute_task_hash(&task.name, &task.cache_files)?;
                self.cache.mark_cached(&task.name, &hash)?;
            }
        } else {
            println!("❌ Task '{}' failed", task.name);
            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
            anyhow::bail!(
                "Task '{}' failed with exit code: {:?}",
                task.name,
                output.status.code()
            );
        }

        Ok(())
    }

    pub async fn execute_task_with_watch(&self, task_name: &str, parallel: bool) -> Result<()> {
        // Run once initially
        println!("🚀 Initial run of task: {task_name}");
        if parallel {
            self.execute_task_parallel(task_name).await?;
        } else {
            self.execute_task(task_name).await?;
        }

        println!("👀 Watching for file changes... (Press Ctrl+C to stop)");

        // Set up file watcher
        let (tx, rx) = mpsc::channel();
        let mut watcher: RecommendedWatcher = Watcher::new(
            move |res: notify::Result<Event>| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            },
            notify::Config::default(),
        )?;

        // Watch current directory
        watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

        // Watch loop
        loop {
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(_event) => {
                    // Debounce: wait a bit for more changes
                    std::thread::sleep(Duration::from_millis(200));

                    // Drain any additional events
                    while rx.try_recv().is_ok() {}

                    println!("\n🔄 File change detected, re-running task: {task_name}");

                    // Clear cache to force rebuild
                    let _ = std::fs::remove_dir_all(".rush-cache");

                    if parallel {
                        if let Err(e) = self.execute_task_parallel(task_name).await {
                            eprintln!("❌ Task failed: {e}");
                        }
                    } else if let Err(e) = self.execute_task(task_name).await {
                        eprintln!("❌ Task failed: {e}");
                    }

                    println!("👀 Watching for more changes...");
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // No events, continue watching
                    tokio::task::yield_now().await;
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }

        Ok(())
    }

    async fn run_single_task_with_progress(&self, task: &Task, progress: &ProgressBar) -> Result<()> {
        let start_time = Instant::now();
        
        // Check cache if cache files are specified
        if !task.cache_files.is_empty() {
            let hash = self
                .cache
                .compute_task_hash(&task.name, &task.cache_files)?;
            if self.cache.is_cached(&task.name, &hash) {
                progress.set_message(format!("⚡ {} (cached)", task.name));
                return Ok(());
            }
        }

        progress.set_message(format!("🏃 Running {}", task.name));

        let mut cmd = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", &task.cmd]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", &task.cmd]);
            cmd
        };

        // Set environment variables
        for (key, value) in &task.env {
            cmd.env(key, value);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd.output().await?;
        let elapsed = start_time.elapsed();

        if output.status.success() {
            progress.set_message(format!("✅ {} ({:.1}s)", task.name, elapsed.as_secs_f32()));
            
            // Cache the result if cache files are specified
            if !task.cache_files.is_empty() {
                let hash = self
                    .cache
                    .compute_task_hash(&task.name, &task.cache_files)?;
                self.cache.mark_cached(&task.name, &hash)?;
            }
        } else {
            progress.set_message(format!("❌ {} failed", task.name));
            if !output.stderr.is_empty() {
                eprintln!("Error output for {}:\n{}", task.name, String::from_utf8_lossy(&output.stderr));
            }
            anyhow::bail!(
                "Task '{}' failed with exit code: {:?}",
                task.name,
                output.status.code()
            );
        }

        Ok(())
    }

    async fn run_task_standalone_with_progress(task: &Task, cache: &TaskCache, progress: &ProgressBar) -> Result<()> {
        let start_time = Instant::now();
        
        // Check cache if cache files are specified
        if !task.cache_files.is_empty() {
            let hash = cache.compute_task_hash(&task.name, &task.cache_files)?;
            if cache.is_cached(&task.name, &hash) {
                progress.set_message(format!("⚡ {} (cached)", task.name));
                return Ok(());
            }
        }

        progress.set_message(format!("🏃 Running {}", task.name));

        let mut cmd = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", &task.cmd]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", &task.cmd]);
            cmd
        };

        // Set environment variables
        for (key, value) in &task.env {
            cmd.env(key, value);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd.output().await?;
        let elapsed = start_time.elapsed();

        if output.status.success() {
            progress.set_message(format!("✅ {} ({:.1}s)", task.name, elapsed.as_secs_f32()));
            
            // Cache the result if cache files are specified
            if !task.cache_files.is_empty() {
                let hash = cache.compute_task_hash(&task.name, &task.cache_files)?;
                cache.mark_cached(&task.name, &hash)?;
            }
        } else {
            progress.set_message(format!("❌ {} failed", task.name));
            if !output.stderr.is_empty() {
                eprintln!("Error output for {}:\n{}", task.name, String::from_utf8_lossy(&output.stderr));
            }
            anyhow::bail!(
                "Task '{}' failed with exit code: {:?}",
                task.name,
                output.status.code()
            );
        }

        Ok(())
    }
}
