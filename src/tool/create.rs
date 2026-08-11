use crate::config::CreatedConfig;
use crate::utils::update_frontmatter_field;
use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};

/// Create: ファイルが存在しない場合に作成する
pub fn create_file_if_not_exists(
    base_directory: &Path,
    file_path: &Path,
    template: &Option<PathBuf>,
    created_config: &Option<CreatedConfig>,
) -> Result<bool> {
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    if !file_path.exists() {
        if let Some(template) = &template {
            let template_path = base_directory.join(template);
            if !template_path.exists() {
                return Err(anyhow!("Not Exist: Template File - {:?}", template_path));
            }
            fs::copy(&template_path, file_path)?;
        } else {
            fs::File::create(file_path)?;
        }
        if let Some(created_config) = &created_config
            && let Some(field_name) = &created_config.field
        {
            read_file_and_update_frontmatter(
                file_path,
                field_name,
                created_config
                    .format
                    .as_deref()
                    .unwrap_or("%Y-%m-%d %H:%M:%S"),
            )?;
        }
    }
    Ok(true)
}

/// Update: YAML frontmatterのフィールドを更新する
fn read_file_and_update_frontmatter(
    file_path: &Path,
    field_name: &str,
    new_value: &str,
) -> Result<()> {
    let contents = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {:?}", file_path))?;
    let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    update_frontmatter_field(&mut lines, field_name, new_value)?;
    fs::write(file_path, lines.join("\n"))
        .with_context(|| format!("Failed to write file: {:?}", file_path))?;
    Ok(())
}
