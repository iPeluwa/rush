mod cache;
mod config;
mod executor;
mod graph;

use anyhow::Result;
use clap::{Arg, Command};
use config::RushConfig;
use executor::TaskExecutor;
use graph::TaskGraph;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("rush")
        .version("0.2.0")
        .about("A modern task runner with parallel execution and intelligent caching")
        .arg(
            Arg::new("task")
                .help("Task to run")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("parallel")
                .short('j')
                .long("parallel")
                .help("Run tasks in parallel where possible")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("List all available tasks with descriptions")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("watch")
                .short('w')
                .long("watch")
                .help("Watch for file changes and re-run task")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let config = RushConfig::find_config()?;
    let graph = TaskGraph::from(&config);
    let executor = TaskExecutor::new(graph);
    
    // Handle --list flag
    if matches.get_flag("list") {
    println!("📋 Available tasks:\n");
    let mut tasks: Vec<_> = config.tasks.iter().collect();
    tasks.sort_by_key(|(name, _)| *name);
    
    for (name, task) in tasks {
            match &task.description {
            Some(desc) => {
                println!("  📦 {}", name);
            println!("     {}", desc);
        println!("     Command: {}", task.cmd);
        if !task.deps.is_empty() {
                println!("     Dependencies: {}", task.deps.join(", "));
                }
                    println!();
                }
                None => {
                    println!("  📦 {}: {}", name, task.cmd);
                    if !task.deps.is_empty() {
                        println!("     Dependencies: {}", task.deps.join(", "));
                    }
                    println!();
                }
            }
        }
        return Ok(());
    }
    
    if let Some(task_name) = matches.get_one::<String>("task") {
        let parallel = matches.get_flag("parallel");
        let watch = matches.get_flag("watch");
        
        if watch {
            println!("🔍 Starting file watcher for task: {}", task_name);
            executor.execute_task_with_watch(task_name, parallel).await?;
        } else if parallel {
            executor.execute_task_parallel(task_name).await?;
        } else {
            executor.execute_task(task_name).await?;
        }
    } else {
        println!("Available tasks:");
        for (name, task) in &config.tasks {
            match &task.description {
                Some(desc) => println!("  {}: {} ({})", name, desc, task.cmd),
                None => println!("  {}: {}", name, task.cmd),
            }
        }
        println!("\nUse 'rush --list' for detailed task information");
    }

    Ok(())
}
