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
        .version("0.1.0")
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
        .get_matches();

    let config = RushConfig::find_config()?;
    let graph = TaskGraph::from(&config);
    let executor = TaskExecutor::new(graph);

    if let Some(task_name) = matches.get_one::<String>("task") {
        let parallel = matches.get_flag("parallel");
        if parallel {
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
    }

    Ok(())
}
