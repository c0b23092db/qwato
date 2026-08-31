use crate::config::Config;
use crate::utils::markdown::{is_blank, is_heading, is_list};
use crate::utils::{
    check_command_exists, conversion_target_file_path, read_file_to_string,
    update_frontmatter_field,
};
use anyhow::{Context, Result, anyhow};
use chrono::Local;
use regex::Regex;
use std::fs;

/// Last Edit: 最後に挿入したメモ行を置き換える
pub fn edit_last_message(
    config: &Config,
    command_name: &str,
    new_message: &str,
    link: Option<&str>,
    clap_tag: &[String],
) -> Result<()> {
    let command = check_command_exists(config, command_name)?;
    let file_path = conversion_target_file_path(config, command_name, &command)?;

    if !file_path.exists() {
        return Err(anyhow!(
            "Target file does not exist: {}",
            file_path.display()
        ));
    }

    let contents = read_file_to_string(&file_path)?;
    let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    if lines.is_empty() {
        return Err(anyhow!("File is empty: {}", file_path.display()));
    }

    let target_index =
        find_last_inserted_index(&lines, command.insert.as_deref(), command.end_line)?;

    // Format: Tags
    let mut tags = command.tags.unwrap_or_default();
    tags.extend(clap_tag.iter().cloned());
    let tag_prefix = if tags.is_empty() {
        String::new()
    } else {
        format!("#{} ", tags.join(" #"))
    };

    // Format: Link
    let formatted_message = if let Some(link) = link {
        format!("[{}]({})", new_message, link)
    } else {
        new_message.to_string()
    };

    let target_line = &lines[target_index];

    // Regex to match prefix: (indent + list symbol + optional checkbox + optional timestamp)
    let re = Regex::new(r"^(\s*(?:[-*+]\s|\d+[.]\s)(?:\[[ xX]\]\s*)?(?:\d{2}:\d{2}:\d{2}\s+)?)")
        .unwrap();

    let new_line = if let Some(caps) = re.captures(target_line) {
        let prefix = &caps[1];
        format!("{}{}{}", prefix, tag_prefix, formatted_message)
    } else {
        format!("- {}{}", tag_prefix, formatted_message)
    };

    lines[target_index] = new_line;

    // Update: Modified Field
    if let Some(modified_config) = &config.modified
        && let Some(field_name) = &modified_config.field
    {
        let _ = update_frontmatter_field(
            &mut lines,
            field_name,
            modified_config
                .format
                .as_deref()
                .unwrap_or("%Y-%m-%d %H:%M:%S"),
        );
    }

    let new_contents = lines.join("\n");
    fs::write(&file_path, new_contents)
        .with_context(|| format!("Failed to write to file: {}", file_path.display()))?;

    Ok(())
}

/// 最後に挿入された行のインデックスを探す
fn find_last_inserted_index(
    lines: &[String],
    insert: Option<&str>,
    end_line: bool,
) -> Result<usize> {
    if let Some(pattern) = insert {
        let now_time = Local::now();
        let insert_pattern = now_time.format(pattern).to_string();

        let anchor = lines
            .iter()
            .rposition(|line| line.trim() == insert_pattern)
            .ok_or_else(|| anyhow!("Not Found: Insert line {:?}", insert_pattern))?;

        let rest = &lines[anchor + 1..];
        let next_heading = rest
            .iter()
            .position(|line| is_heading(line))
            .map(|idx| anchor + 1 + idx)
            .unwrap_or(lines.len());

        if !end_line {
            // 先頭側の最初のリスト項目
            for (i, line) in lines.iter().enumerate().take(next_heading).skip(anchor + 1) {
                if is_list(line) {
                    return Ok(i);
                }
            }
        } else {
            // 末尾側の最後のリスト項目
            for i in (anchor + 1..next_heading).rev() {
                if is_list(&lines[i]) {
                    return Ok(i);
                }
            }
        }
    } else if !end_line {
        // Frontmatterの直後以降で最初のリスト項目
        let start = lines
            .iter()
            .rposition(|line| line.trim() == "---")
            .map(|index| index + 1)
            .unwrap_or(0);

        for (i, line) in lines.iter().enumerate().skip(start) {
            if is_list(line) {
                return Ok(i);
            }
        }
    } else {
        // ファイル全体の最後のリスト項目
        for i in (0..lines.len()).rev() {
            if is_list(&lines[i]) {
                return Ok(i);
            }
        }
    }

    // もしリスト項目が見つからなかった場合、最後の非空行を探す
    for i in (0..lines.len()).rev() {
        if !is_blank(&lines[i]) && !is_heading(&lines[i]) && lines[i].trim() != "---" {
            return Ok(i);
        }
    }

    Err(anyhow!("No message entry found to edit"))
}
