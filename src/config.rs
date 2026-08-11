use anyhow::{Context, Result};
use dirs::home_dir;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub base_directory: PathBuf,
    #[serde(default)]
    pub default_command: String,
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

#[derive(Debug, Clone, Deserialize)]
pub struct ListConfig {
    #[serde(default)]
    pub limit: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatedConfig {
    #[serde(default)]
    pub field: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifiedConfig {
    #[serde(default)]
    pub field: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandConfig {
    #[serde(default)]
    pub auto_create: bool,
    #[serde(default)]
    pub directory: Option<PathBuf>,
    #[serde(default)]
    pub template: Option<PathBuf>,
    #[serde(default)]
    pub file: Option<PathBuf>,
    #[serde(default)]
    pub insert: Option<String>,
    #[serde(default)]
    pub end_line: bool,
    #[serde(default)]
    pub not_format: bool,
}

impl Default for Config {
    fn default() -> Self {
        let mut command = HashMap::new();
        command.insert(
            "default".to_string(),
            CommandConfig {
                auto_create: true,
                directory: None,
                file: Some(PathBuf::from("%Y-%m-%d")),
                template: None,
                insert: None,
                end_line: false,
                not_format: false,
            },
        );
        Self {
            base_directory: PathBuf::from("~/Documents/Qwato"),
            default_command: "default".to_string(),
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
            command,
        }
    }
}

impl Config {
    pub fn set_limit(&mut self, limit: usize) {
        if self.list.is_none() {
            self.list = Some(ListConfig { limit });
        } else {
            self.list.as_mut().unwrap().limit = limit;
        }
    }
}

pub fn load_config() -> Result<Config> {
    let config_path = home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join(".config")
        .join("qwato")
        .join("config.toml");
    if !config_path.exists() {
        return Ok(Config::default());
    }
    let content = read_config(&config_path)?;
    let config = parse_config(&content)?;
    Ok(config)
}

/// Read: 指定された設定ファイル
fn read_config(config_path: &Path) -> Result<String> {
    std::fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))
}

/// Parse: 設定ファイル
fn parse_config(config_content: &str) -> Result<Config> {
    toml::from_str(config_content).with_context(|| "Failed to parse config file")
}
