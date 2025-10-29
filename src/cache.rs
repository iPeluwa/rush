use anyhow::Result;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::task;

const CACHE_VERSION: &str = "taskrush-cache-v1";

#[derive(Debug, Clone)]
pub struct TaskCache {
    cache_dir: PathBuf,
}

impl TaskCache {
    pub fn new() -> Self {
        Self {
            cache_dir: PathBuf::from(".rush-cache"),
        }
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    pub async fn compute_task_hash(
        &self,
        task_name: &str,
        command: &str,
        env: &HashMap<String, String>,
        dependencies: &[String],
        cache_files: &[String],
    ) -> Result<String> {
        let task_name = task_name.to_string();
        let command = command.to_string();
        let mut env_pairs: Vec<_> = env.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        env_pairs.sort_by(|a, b| a.0.cmp(&b.0));

        let mut dependency_list = dependencies.to_vec();
        dependency_list.sort();

        let mut cache_inputs = cache_files.to_vec();
        cache_inputs.sort();

        task::spawn_blocking(move || -> Result<String> {
            let mut hasher = Sha256::new();

            hasher.update(CACHE_VERSION.as_bytes());
            hasher.update(task_name.as_bytes());
            hasher.update(command.as_bytes());

            for (key, value) in env_pairs {
                hasher.update(key.as_bytes());
                hasher.update(b"=");
                hasher.update(value.as_bytes());
            }

            for dep in dependency_list {
                hasher.update(dep.as_bytes());
            }

            for file_path in cache_inputs {
                let path = Path::new(&file_path);
                if path.exists() {
                    let content = std::fs::read(path)?;
                    hasher.update(&content);
                } else {
                    hasher.update(b"<file-not-found>");
                    hasher.update(file_path.as_bytes());
                }
            }

            let result = hasher.finalize();
            Ok(format!("{result:x}"))
        })
        .await?
    }

    pub async fn is_cached(&self, task_name: &str, hash: &str) -> Result<bool> {
        let cache_dir = self.cache_dir.clone();
        let task_name = task_name.to_string();
        let hash = hash.to_string();

        task::spawn_blocking(move || -> Result<bool> {
            let cache_file = cache_dir.join(format!("{}.{}", task_name, hash));
            Ok(cache_file.exists())
        })
        .await?
    }

    pub async fn mark_cached(&self, task_name: &str, hash: &str) -> Result<()> {
        let cache_dir = self.cache_dir.clone();
        let task_name = task_name.to_string();
        let hash = hash.to_string();

        task::spawn_blocking(move || -> Result<()> {
            std::fs::create_dir_all(&cache_dir)?;

            if let Ok(entries) = std::fs::read_dir(&cache_dir) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.starts_with(&format!("{task_name}.")) {
                        let _ = std::fs::remove_file(entry.path());
                    }
                }
            }

            let cache_file = cache_dir.join(format!("{}.{}", task_name, hash));
            std::fs::write(&cache_file, b"")?;

            Ok(())
        })
        .await?
    }

    pub async fn invalidate_task(&self, task_name: &str) -> Result<()> {
        let cache_dir = self.cache_dir.clone();
        let task_name = task_name.to_string();

        task::spawn_blocking(move || -> Result<()> {
            if !cache_dir.exists() {
                return Ok(());
            }

            if let Ok(entries) = std::fs::read_dir(&cache_dir) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name();
                    if file_name
                        .to_string_lossy()
                        .starts_with(&format!("{task_name}."))
                    {
                        let _ = std::fs::remove_file(entry.path());
                    }
                }
            }

            Ok(())
        })
        .await?
    }
}
