use anyhow::{Context, Result};
use chrono::NaiveDate;
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
    pub date_format: Option<DailyFile>,
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
    pub base_directory: Option<PathBuf>,
    #[serde(default)]
    pub auto_create: bool,
    #[serde(default)]
    pub date_format: Option<DailyFile>,
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

impl CommandConfig {
    pub fn date_format(&self) -> DailyFile {
        self.date_format.clone().unwrap_or(DailyFile::Line)
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub enum DailyFile {
    /// YYYY-MM-DD.md
    #[default]
    #[serde(rename = "line")]
    Line,
    /// YYYY/MM/DD.md
    #[serde(rename = "slash")]
    Slash,
    /// YYYY/MM/DD/YYYY-MM-DD.md
    #[serde(rename = "slash_line")]
    SlashLine,
    /// file-name.md / Header for YYYY-MM-DD
    #[serde(rename = "header")]
    Header,
    /// file-name.md
    #[serde(rename = "onefile")]
    Onefile,
}

impl DailyFile {
    pub fn parse_date_from_path(&self, file_path: &Path) -> Option<NaiveDate> {
        let stem = file_path.file_stem()?.to_str()?;

        match self {
            DailyFile::Line => NaiveDate::parse_from_str(stem, "%Y-%m-%d").ok(),
            DailyFile::Slash => {
                let parent = file_path.parent()?;
                let month = parent.file_name()?.to_str()?;
                let year = parent.parent()?.file_name()?.to_str()?;

                if month.len() == 2
                    && month.chars().all(|c| c.is_ascii_digit())
                    && year.len() == 4
                    && year.chars().all(|c| c.is_ascii_digit())
                {
                    let formatted = format!("{year}/{month}/{stem}");
                    NaiveDate::parse_from_str(&formatted, "%Y/%m/%d").ok()
                } else {
                    None
                }
            }
            DailyFile::SlashLine => {
                let parent = file_path.parent()?;
                let month = parent.file_name()?.to_str()?;
                let year = parent.parent()?.file_name()?.to_str()?;

                if month.len() == 2
                    && month.chars().all(|c| c.is_ascii_digit())
                    && year.len() == 4
                    && year.chars().all(|c| c.is_ascii_digit())
                {
                    let formatted = format!("{year}/{month}/{stem}");
                    NaiveDate::parse_from_str(&formatted, "%Y/%m/%d")
                        .ok()
                        .or_else(|| NaiveDate::parse_from_str(&formatted, "%Y/%m/%Y-%m-%d").ok())
                } else {
                    None
                }
            }
            DailyFile::Header | DailyFile::Onefile => None,
        }
    }
}

impl Config {
    fn new() -> Self {
        Self {
            no_global: false,
            base_directory: Some(home_dir().unwrap_or(PathBuf::from("~"))),
            default_command: None,
            auto_command: false,
            date_format: Some(DailyFile::Line),
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
    pub fn apply_base_directory_to_commands(&mut self) {
        if let Some(ref base_dir) = self.base_directory {
            for command in self.command.values_mut() {
                if command.base_directory.is_none() {
                    command.base_directory = Some(base_dir.clone());
                }
            }
        }
    }
}

/// Load: Config Files
pub fn load_config(config_path: &Option<PathBuf>) -> Result<Config> {
    if let Some(config_path) = config_path
        && !config_path.exists()
    {
        anyhow::bail!("Failed to Find: {}", config_path.display())
    };
    let Ok(config_paths) = find_config_path(config_path) else {
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
/// 1. Optional Config Path: --config <config_path>
/// 2. Current directory: ./qwato.toml
/// 3. Home directory: ~/.config/qwato/*.toml
fn find_config_path(path: &Option<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut config_paths = Vec::new();

    // Config directory: ~/.config/qwato/*.toml
    let home_config_dir = home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join(".config")
        .join("qwato");
    if home_config_dir.is_dir()
        && let Ok(entries) = std::fs::read_dir(&home_config_dir)
    {
        let mut toml_files = Vec::new();
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() && p.extension().is_some_and(|ext| ext == "toml") {
                toml_files.push(p);
            }
        }
        toml_files.sort();
        config_paths.extend(toml_files);
    }

    // Current directory: ./qwato.toml
    let current_config_path = current_dir()
        .with_context(|| "Failed to Get: current directory")?
        .join("qwato.toml");
    if current_config_path.exists() {
        config_paths.push(current_config_path);
    }

    // Optional Config Path: --config <config_path>
    if let Some(config_path) = path {
        if config_path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(config_path) {
                let mut toml_files = Vec::new();
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.is_file() && p.extension().is_some_and(|ext| ext == "toml") {
                        toml_files.push(p);
                    }
                }
                toml_files.sort();
                config_paths.extend(toml_files);
            }
        } else {
            config_paths.push(config_path.clone());
        }
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
    let mut config: Config =
        toml::from_str(config_content).with_context(|| "Failed to Parse: config file")?;
    config.apply_base_directory_to_commands();
    Ok(config)
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
    base_config.date_format = override_config.date_format.or(base_config.date_format);
    base_config.list = override_config.list.or(base_config.list);
    base_config.time_format = override_config.time_format.or(base_config.time_format);
    base_config.created = override_config.created.or(base_config.created);
    base_config.modified = override_config.modified.or(base_config.modified);
    for (k, v) in override_config.command {
        base_config.command.insert(k, v);
    }
    base_config
}
