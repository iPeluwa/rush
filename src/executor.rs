use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::time::sleep;

use crate::cache::TaskCache;
use crate::graph::{Task, TaskGraph};

pub struct TaskExecutor {
    graph: TaskGraph,
    cache: TaskCache,
    verbose: bool,
    workspace_root: PathBuf,
}

impl TaskExecutor {
    pub fn new(graph: TaskGraph, verbose: bool) -> Self {
        let workspace_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self {
            graph,
            cache: TaskCache::new(),
            verbose,
            workspace_root,
        }
    }

    pub async fn execute_task(&self, task_name: &str) -> Result<()> {
        // Validate task exists
        if !self.graph.tasks.contains_key(task_name) {
            return Err(anyhow::anyhow!(
                "❌ Task '{}' not found. Available tasks: {}",
                task_name,
                self.graph
                    .tasks
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        let execution_order = self
            .graph
            .topological_sort(task_name)
            .with_context(|| format!("Failed to resolve dependencies for task '{task_name}'"))?;

        if self.verbose {
            println!("🔍 Debug: Task dependency resolution");
            println!("   Target task: {task_name}");
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
                self.run_single_task(task)
                    .await
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

                    self.run_single_task_with_progress(task, &task_progress)
                        .await?;
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
                            let result = Self::run_task_standalone_with_progress(
                                &task,
                                &cache,
                                &task_progress,
                            )
                            .await;
                            if result.is_ok() {
                                task_progress
                                    .finish_with_message(format!("✅ {} completed", task.name));
                            } else {
                                task_progress
                                    .finish_with_message(format!("❌ {} failed", task.name));
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

            let ready_set: HashSet<_> = ready_tasks.iter().cloned().collect();

            for task in &ready_set {
                completed.insert(task.clone());
            }
            remaining.retain(|task| !ready_set.contains(task));

            levels.push(ready_tasks);
        }

        Ok(levels)
    }

    async fn run_single_task(&self, task: &Task) -> Result<()> {
        Self::run_task_with_handlers(
            &self.cache,
            task,
            |task| println!("⚡ Task '{}' skipped (cached)", task.name),
            |task| println!("🏃 Running task: {}", task.name),
            |task, output, _elapsed| {
                println!("✅ Task '{}' completed successfully", task.name);
                if !output.stdout.is_empty() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
            },
            |task, output| {
                println!("❌ Task '{}' failed", task.name);
                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            },
        )
        .await
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

        let (tx, mut rx) = mpsc::unbounded_channel();
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

        let cache_root = self.normalize_to_workspace(self.cache.cache_dir());

        while let Some(event) = rx.recv().await {
            if self.should_ignore_event(&event, &cache_root) {
                continue;
            }

            let mut aggregated_events = vec![event];
            let debounce = sleep(Duration::from_millis(200));
            tokio::pin!(debounce);

            loop {
                tokio::select! {
                    _ = &mut debounce => break,
                    maybe_event = rx.recv() => {
                        match maybe_event {
                            Some(next_event) => {
                                if !self.should_ignore_event(&next_event, &cache_root) {
                                    aggregated_events.push(next_event);
                                }
                            }
                            None => return Ok(()),
                        }
                    }
                }
            }

            let changed_paths: Vec<PathBuf> = aggregated_events
                .into_iter()
                .flat_map(|event| event.paths.into_iter())
                .collect();

            if changed_paths.is_empty() {
                continue;
            }

            let mut affected_tasks: Vec<String> = self
                .affected_tasks_for_paths(&changed_paths, &cache_root)
                .into_iter()
                .collect();
            affected_tasks.sort();

            for task in &affected_tasks {
                self.cache.invalidate_task(task).await?;
            }

            println!("\n🔄 File change detected, re-running task: {task_name}");

            if parallel {
                if let Err(e) = self.execute_task_parallel(task_name).await {
                    eprintln!("❌ Task failed: {e}");
                }
            } else if let Err(e) = self.execute_task(task_name).await {
                eprintln!("❌ Task failed: {e}");
            }

            if !affected_tasks.is_empty() {
                println!("   Cache invalidated for: {}", affected_tasks.join(", "));
            }

            println!("👀 Watching for more changes...");
        }

        Ok(())
    }

    async fn run_single_task_with_progress(
        &self,
        task: &Task,
        progress: &ProgressBar,
    ) -> Result<()> {
        Self::run_task_with_handlers(
            &self.cache,
            task,
            |task| progress.set_message(format!("⚡ {} (cached)", task.name)),
            |task| progress.set_message(format!("🏃 Running {}", task.name)),
            |task, _output, elapsed| {
                progress.set_message(format!("✅ {} ({:.1}s)", task.name, elapsed.as_secs_f32()));
            },
            |task, output| {
                progress.set_message(format!("❌ {} failed", task.name));
                if !output.stderr.is_empty() {
                    eprintln!(
                        "Error output for {}:\n{}",
                        task.name,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            },
        )
        .await
    }

    async fn run_task_standalone_with_progress(
        task: &Task,
        cache: &TaskCache,
        progress: &ProgressBar,
    ) -> Result<()> {
        Self::run_task_with_handlers(
            cache,
            task,
            |task| progress.set_message(format!("⚡ {} (cached)", task.name)),
            |task| progress.set_message(format!("🏃 Running {}", task.name)),
            |task, _output, elapsed| {
                progress.set_message(format!("✅ {} ({:.1}s)", task.name, elapsed.as_secs_f32()));
            },
            |task, output| {
                progress.set_message(format!("❌ {} failed", task.name));
                if !output.stderr.is_empty() {
                    eprintln!(
                        "Error output for {}:\n{}",
                        task.name,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            },
        )
        .await
    }

    fn normalize_to_workspace(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.workspace_root.join(path)
        }
    }

    fn should_ignore_event(&self, event: &Event, cache_root: &Path) -> bool {
        if event.paths.is_empty() {
            return false;
        }

        event
            .paths
            .iter()
            .all(|path| self.normalize_to_workspace(path).starts_with(cache_root))
    }

    fn affected_tasks_for_paths(&self, paths: &[PathBuf], cache_root: &Path) -> HashSet<String> {
        let mut affected = HashSet::new();

        for path in paths {
            let absolute_path = self.normalize_to_workspace(path);

            if absolute_path.starts_with(cache_root) {
                continue;
            }

            for (name, task) in &self.graph.tasks {
                if task.cache_files.is_empty() {
                    continue;
                }

                if task.cache_files.iter().any(|cache_file| {
                    let cache_path = Path::new(cache_file);
                    self.normalize_to_workspace(cache_path) == absolute_path
                }) {
                    affected.insert(name.clone());
                }
            }
        }

        affected
    }
}

impl TaskExecutor {
    async fn run_task_with_handlers<FCacheHit, FStart, FSuccess, FFailure>(
        cache: &TaskCache,
        task: &Task,
        mut on_cache_hit: FCacheHit,
        mut on_start: FStart,
        mut on_success: FSuccess,
        mut on_failure: FFailure,
    ) -> Result<()>
    where
        FCacheHit: FnMut(&Task) + Send,
        FStart: FnMut(&Task) + Send,
        FSuccess: FnMut(&Task, &std::process::Output, Duration) + Send,
        FFailure: FnMut(&Task, &std::process::Output) + Send,
    {
        let mut cached_hash: Option<String> = None;

        if !task.cache_files.is_empty() {
            let hash = cache
                .compute_task_hash(
                    &task.name,
                    &task.cmd,
                    &task.env,
                    &task.deps,
                    &task.cache_files,
                )
                .await?;
            if cache.is_cached(&task.name, &hash).await? {
                on_cache_hit(task);
                return Ok(());
            }
            cached_hash = Some(hash);
        }

        on_start(task);
        let start_time = Instant::now();

        let mut cmd = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", &task.cmd]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", &task.cmd]);
            cmd
        };

        for (key, value) in &task.env {
            cmd.env(key, value);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd.output().await?;
        let elapsed = start_time.elapsed();

        if output.status.success() {
            if let Some(hash) = cached_hash {
                cache.mark_cached(&task.name, &hash).await?;
            }
            on_success(task, &output, elapsed);
            Ok(())
        } else {
            on_failure(task, &output);
            anyhow::bail!(
                "Task '{}' failed with exit code: {:?}",
                task.name,
                output.status.code()
            );
        }
    }
}
