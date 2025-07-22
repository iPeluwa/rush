use anyhow::{bail, Result};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct TaskGraph {
    pub tasks: HashMap<String, Task>,
    pub dependencies: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub cmd: String,
    pub env: HashMap<String, String>,
    pub cache_files: Vec<String>,
}

impl TaskGraph {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, name: String, task: Task) {
        self.tasks.insert(name.clone(), task);
        self.dependencies.entry(name).or_insert_with(Vec::new);
    }

    pub fn add_dependency(&mut self, task: String, dependency: String) {
        self.dependencies.entry(task).or_default().push(dependency);
    }

    pub fn topological_sort(&self, start_task: &str) -> Result<Vec<String>> {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        let mut temp_visited = HashSet::new();

        self.dfs_visit(start_task, &mut visited, &mut temp_visited, &mut stack)?;
        
        Ok(stack)
    }

    fn dfs_visit(
        &self,
        task: &str,
        visited: &mut HashSet<String>,
        temp_visited: &mut HashSet<String>,
        stack: &mut Vec<String>,
    ) -> Result<()> {
        if temp_visited.contains(task) {
            bail!("Circular dependency detected involving task: {}", task);
        }

        if visited.contains(task) {
            return Ok(());
        }

        if !self.tasks.contains_key(task) {
            bail!("Task '{}' not found", task);
        }

        temp_visited.insert(task.to_string());

        if let Some(deps) = self.dependencies.get(task) {
            for dep in deps {
                self.dfs_visit(dep, visited, temp_visited, stack)?;
            }
        }

        temp_visited.remove(task);
        visited.insert(task.to_string());
        stack.push(task.to_string());

        Ok(())
    }
}

impl From<&crate::config::RushConfig> for TaskGraph {
    fn from(config: &crate::config::RushConfig) -> Self {
        let mut graph = TaskGraph::new();

        for (name, task_config) in &config.tasks {
            let task = Task {
                name: name.clone(),
                cmd: task_config.cmd.clone(),
                env: task_config.env.clone(),
                cache_files: task_config.cache.clone(),
            };

            graph.add_task(name.clone(), task);

            for dep in &task_config.deps {
                graph.add_dependency(name.clone(), dep.clone());
            }
        }

        graph
    }
}
