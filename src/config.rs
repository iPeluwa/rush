use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RushConfig {
    pub tasks: HashMap<String, Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub cmd: String,
    #[serde(default)]
    pub deps: Vec<String>,
    #[serde(default)]
    pub cache: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl RushConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: RushConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn find_config() -> Result<RushConfig> {
        let candidates = [".rush", ".rush.yml", ".rush.yaml"];
        
        for candidate in &candidates {
            if Path::new(candidate).exists() {
                return Self::load(candidate);
            }
        }
        
        anyhow::bail!("No .rush config file found in current directory")
    }
}
