mod config;

use anyhow::Result;
use clap::{Arg, Command};
use config::RushConfig;

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
    
    if let Some(task_name) = matches.get_one::<String>("task") {
        println!("Running task: {}", task_name);
        // TODO: Execute specific task
    } else {
        println!("Available tasks:");
        for (name, task) in &config.tasks {
            println!("  {}: {}", name, task.cmd);
        }
    }

    Ok(())
}
