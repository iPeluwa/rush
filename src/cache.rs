use anyhow::Result;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct TaskCache {
    cache_dir: String,
}

impl TaskCache {
    pub fn new() -> Self {
        Self {
            cache_dir: ".rush-cache".to_string(),
        }
    }

    pub fn ensure_cache_dir(&self) -> Result<()> {
        fs::create_dir_all(&self.cache_dir)?;
        Ok(())
    }

    pub fn compute_task_hash(&self, task_name: &str, cache_files: &[String]) -> Result<String> {
        let mut hasher = Sha256::new();

        // Hash the task name
        hasher.update(task_name.as_bytes());

        // Hash each cache file's content
        for file_path in cache_files {
            if Path::new(file_path).exists() {
                let content = fs::read(file_path)?;
                hasher.update(&content);
            } else {
                // Hash the fact that the file doesn't exist
                hasher.update(b"<file-not-found>");
            }
        }

        let result = hasher.finalize();
        Ok(format!("{result:x}"))
    }

    pub fn is_cached(&self, task_name: &str, hash: &str) -> bool {
        let cache_file = format!("{}/{}.{}", self.cache_dir, task_name, hash);
        Path::new(&cache_file).exists()
    }

    pub fn mark_cached(&self, task_name: &str, hash: &str) -> Result<()> {
        self.ensure_cache_dir()?;

        // Remove old cache files for this task
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                if file_name_str.starts_with(&format!("{task_name}.")) {
                    let _ = fs::remove_file(entry.path());
                }
            }
        }

        // Create new cache marker
        let cache_file = format!("{}/{}.{}", self.cache_dir, task_name, hash);
        fs::write(cache_file, "")?;

        Ok(())
    }
}
