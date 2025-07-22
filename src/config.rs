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
    #[serde(default)]
    pub description: Option<String>,
}

impl RushConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let expanded_content = Self::expand_env_vars(&content);
        let config: RushConfig = serde_yaml::from_str(&expanded_content)?;
        Ok(config)
    }

    fn expand_env_vars(content: &str) -> String {
        // Handle ${VAR:-default} syntax
        let default_regex = regex::Regex::new(r"\$\{([^}]+):-([^}]*)\}").unwrap();
        let mut expanded = default_regex
            .replace_all(content, |caps: &regex::Captures| {
                let var_name = &caps[1];
                let default_value = &caps[2];
                std::env::var(var_name).unwrap_or_else(|_| default_value.to_string())
            })
            .to_string();

        // Handle regular ${VAR} syntax
        let var_regex = regex::Regex::new(r"\$\{([^}]+)\}").unwrap();
        expanded = var_regex
            .replace_all(&expanded, |caps: &regex::Captures| {
                let var_name = &caps[1];
                std::env::var(var_name).unwrap_or_else(|_| format!("${{{var_name}}}"))
            })
            .to_string();

        expanded
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
