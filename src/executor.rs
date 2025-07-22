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
            cache: TaskCache::new() 
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
        // For now, just use sequential execution with a note
        println!("🚧 Parallel execution coming soon - running sequentially for now");
        self.execute_task(task_name).await
    }

    async fn run_single_task(&self, task: &Task) -> Result<()> {
        // Check cache if cache files are specified
        if !task.cache_files.is_empty() {
            let hash = self.cache.compute_task_hash(&task.name, &task.cache_files)?;
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

        cmd.stdout(Stdio::piped())
           .stderr(Stdio::piped());

        let output = cmd.output().await?;

        if output.status.success() {
            println!("✅ Task '{}' completed successfully", task.name);
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            
            // Cache the result if cache files are specified
            if !task.cache_files.is_empty() {
                let hash = self.cache.compute_task_hash(&task.name, &task.cache_files)?;
                self.cache.mark_cached(&task.name, &hash)?;
            }
        } else {
            println!("❌ Task '{}' failed", task.name);
            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
            anyhow::bail!("Task '{}' failed with exit code: {:?}", task.name, output.status.code());
        }

        Ok(())
    }
}
