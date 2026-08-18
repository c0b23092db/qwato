use anyhow::{Context, Result};
use dirs::home_dir;
use serde::Deserialize;
use std::collections::HashMap;
use std::env::current_dir;
use std::path::{Path, PathBuf};

/// General Configuration for the Application
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub no_global: bool,
    #[serde(default)]
    pub base_directory: Option<PathBuf>,
    #[serde(default)]
    pub default_command: Option<String>,
    #[serde(default)]
    pub auto_command: bool,
    #[serde(default)]
    pub list: Option<ListConfig>,
    #[serde(default)]
    pub time_format: Option<String>,
    #[serde(default)]
    pub created: Option<CreatedConfig>,
    #[serde(default)]
    pub modified: Option<ModifiedConfig>,
    #[serde(default)]
    pub command: HashMap<String, CommandConfig>,
}

/// Configuration for List
#[derive(Debug, Clone, Deserialize)]
pub struct ListConfig {
    #[serde(default)]
    pub limit: usize,
}

/// Configuration for Created
#[derive(Debug, Clone, Deserialize)]
pub struct CreatedConfig {
    #[serde(default)]
    pub field: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
}

/// Configuration for Modified
#[derive(Debug, Clone, Deserialize)]
pub struct ModifiedConfig {
    #[serde(default)]
    pub field: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
}

/// Configuration for Command
#[derive(Debug, Clone, Deserialize)]
pub struct CommandConfig {
    #[serde(default)]
    pub auto_create: bool,
    #[serde(default)]
    pub template: Option<PathBuf>,
    #[serde(default)]
    pub directory: Option<PathBuf>,
    #[serde(default)]
    pub file: Option<PathBuf>,
    #[serde(default)]
    pub insert: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub not_format: bool,
    #[serde(default)]
    pub end_line: bool,
}

impl Config {
    fn new() -> Self {
        Self {
            no_global: false,
            base_directory: Some(home_dir().unwrap_or(PathBuf::from("~"))),
            default_command: None,
            auto_command: false,
            list: Some(ListConfig { limit: 10 }),
            time_format: Some("%H:%M:%S".to_string()),
            created: Some(CreatedConfig {
                field: None,
                format: Some("%Y-%m-%d %H:%M:%S".to_string()),
            }),
            modified: Some(ModifiedConfig {
                field: None,
                format: Some("%Y-%m-%d %H:%M:%S".to_string()),
            }),
            command: HashMap::new(),
        }
    }
    fn default() -> Result<Config> {
        parse_config(
            r#"
        base_directory = "~/Documents/Qwato"
        default_command = "default"
        time_format = "%H:%M:%S"
        [list]
        limit = 10
        [created]
        format = "%Y-%m-%d %H:%M:%S"
        [modified]
        format = "%Y-%m-%d %H:%M:%S"
        [command.default]
        auto_create = true
        file = "%Y-%m-%d.md"
        not_format = false
        end_line = false
        "#,
        )
    }
    pub fn set_limit(&mut self, limit: usize) {
        if let Some(ref mut list) = self.list {
            list.limit = limit;
        } else {
            self.list = Some(ListConfig { limit });
        }
    }
}

pub fn load_config() -> Result<Config> {
    let Ok(config_paths) = find_config_path() else {
        return Config::default();
    };
    let mut config = Config::new();
    for (count, config_path) in config_paths.iter().enumerate() {
        let content = read_config(config_path)?;
        let load_config = parse_config(&content)?;
        if count == config_paths.len() || !load_config.no_global {
            config = merge_config(config, load_config);
        }
    }
    Ok(config)
}

/// Search: Config File Path
/// 1. Current directory: ./qwato.toml
/// 2. Home directory: ~/.config/qwato/config.toml
fn find_config_path() -> Result<Vec<PathBuf>> {
    let mut config_paths = Vec::new();
    // Home directory: ~/.config/qwato/config.toml
    let config_path = home_dir()
        .unwrap_or_else(|| home_dir().unwrap_or(PathBuf::from("~")))
        .join(".config")
        .join("qwato")
        .join("config.toml");
    if config_path.exists() {
        config_paths.push(config_path);
    }
    // Current directory: ./qwato.toml
    let current_config_path = current_dir()
        .with_context(|| "Failed to Get: current directory")?
        .join("qwato.toml");
    if current_config_path.exists() {
        config_paths.push(current_config_path);
    }
    if config_paths.is_empty() {
        anyhow::bail!("Failed to Find: config file");
    }
    Ok(config_paths)
}

/// Read: Config File
fn read_config(config_path: &Path) -> Result<String> {
    std::fs::read_to_string(config_path)
        .with_context(|| format!("Failed to Read: config file - {}", config_path.display()))
}

/// Parse: Config File Content
fn parse_config(config_content: &str) -> Result<Config> {
    toml::from_str(config_content).with_context(|| "Failed to Parse: config file")
}

/// Merge: Two Configurations
fn merge_config(mut base_config: Config, override_config: Config) -> Config {
    base_config.base_directory = override_config
        .base_directory
        .or(base_config.base_directory);
    base_config.default_command = override_config
        .default_command
        .or(base_config.default_command);
    base_config.auto_command = override_config.auto_command || base_config.auto_command;
    base_config.list = override_config.list.or(base_config.list);
    base_config.time_format = override_config.time_format.or(base_config.time_format);
    base_config.created = override_config.created.or(base_config.created);
    base_config.modified = override_config.modified.or(base_config.modified);
    for (k, v) in override_config.command {
        base_config.command.insert(k, v);
    }
    base_config
}
