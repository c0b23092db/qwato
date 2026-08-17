use crate::config::{CommandConfig, Config};
use anyhow::{Result, anyhow};
use chrono::Local;
use dirs::home_dir;
use std::path::PathBuf;

/// Change: '~'をユーザーのホームディレクトリに変換
pub fn expand_home(path: &PathBuf) -> Result<PathBuf> {
    if path.starts_with("~") {
        let home = home_dir().ok_or_else(|| anyhow!("Failed to get home directory"))?;
        let expanded_path = path
            .to_string_lossy()
            .replacen("~", home.to_str().unwrap(), 1);
        Ok(PathBuf::from(expanded_path))
    } else {
        Ok(PathBuf::from(path))
    }
}

/// Check: 指定したコマンドが存在するかどうかを確認
pub fn check_command_exists(config: &Config, command_name: &str) -> Result<CommandConfig> {
    if let Some(command) = config.command.get(command_name) {
        Ok(command.clone())
    } else {
        Err(anyhow!("unknown command {:?}", command_name))
    }
}

/// Conversion: コマンドに対応するファイルパス → Base Directory + Command Directory + Command File
pub fn conversion_target_file_path(
    config: &Config,
    command_name: &str,
    command: &CommandConfig,
) -> Result<PathBuf> {
    let base_directory = expand_home(
        config
            .base_directory
            .as_ref()
            .unwrap_or(&PathBuf::default()),
    )?;
    let now = Local::now();
    let command_directory = command
        .directory
        .as_deref()
        .map(|pattern| now.format(&pattern.to_string_lossy()).to_string());
    let command_file = command
        .file
        .as_deref()
        .map(|pattern| now.format(&pattern.to_string_lossy()).to_string())
        .unwrap_or_else(|| format!("{command_name}.md"));
    let target_path = base_directory
        .join(command_directory.as_deref().unwrap_or(""))
        .join(command_file);
    Ok(target_path)
}

/// Conversion: コマンドに対応するファイルパス → Base Directory + Command Directory
pub fn conversion_target_directory_path(
    config: &Config,
    command: &CommandConfig,
) -> Result<PathBuf> {
    let base_directory = expand_home(
        config
            .base_directory
            .as_ref()
            .unwrap_or(&PathBuf::default()),
    )?;
    let now = Local::now();
    let command_directory = command
        .directory
        .as_deref()
        .map(|pattern| now.format(&pattern.to_string_lossy()).to_string());
    let target_path = base_directory.join(command_directory.as_deref().unwrap_or(""));
    Ok(target_path)
}

/// Update: YAML frontmatterが存在する場合だけ、指定フィールドを更新する。
///
/// - Frontmatterがない場合: 何もしない
/// - Frontmatterはあるがフィールドがない場合: エラー
pub fn update_frontmatter_field(
    lines: &mut [String],
    field_name: &str,
    format_pattern: &str,
) -> Result<()> {
    let Some(frontmatter_start) = lines.first().filter(|line| line.trim() == "---").map(|_| 0)
    else {
        return Ok(());
    };
    let Some(relative_end) = lines
        .iter()
        .skip(frontmatter_start + 1)
        .position(|line| line.trim() == "---")
    else {
        return Ok(());
    };
    let frontmatter_end = frontmatter_start + 1 + relative_end;
    let new_value = Local::now().format(format_pattern).to_string();
    for line in &mut lines[frontmatter_start + 1..frontmatter_end] {
        let trimmed = line.trim_start();
        let Some((key, _old_value)) = trimmed.split_once(':') else {
            continue;
        };
        if key.trim() != field_name {
            continue;
        }
        let indent_length = line.len() - line.trim_start().len();
        let indent = line[..indent_length].to_owned();
        *line = format!("{indent}{field_name}: {new_value}");
        return Ok(());
    }
    Err(anyhow!("Frontmatter field not found: {:?}", field_name))
}
