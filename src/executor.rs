use anyhow::Result;
use std::process::Stdio;
use tokio::process::Command;

use crate::cache::TaskCache;
use crate::graph::{Task, TaskGraph};

pub struct TaskExecutor {
    graph: TaskGraph,
    cache: TaskCache,
}

impl TaskExecutor {
    pub fn new(graph: TaskGraph) -> Self {
        Self {
            graph,
            cache: TaskCache::new(),
        }
    }

    pub async fn execute_task(&self, task_name: &str) -> Result<()> {
        let execution_order = self.graph.topological_sort(task_name)?;

        println!("Execution order: {}", execution_order.join(" -> "));

        for task_name in execution_order {
            if let Some(task) = self.graph.tasks.get(&task_name) {
                self.run_single_task(task).await?;
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
                    self.run_single_task(task).await?;
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
                        let cache = TaskCache::new(); // Each task gets its own cache instance

                        let handle =
                            tokio::spawn(
                                async move { Self::run_task_standalone(&task, &cache).await },
                            );

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
}
