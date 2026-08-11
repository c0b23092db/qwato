use crate::config::Config;
use crate::tool::create_file_if_not_exists;
use crate::utils::{
    check_command_exists, conversion_target_file_path, expand_home, update_frontmatter_field,
};

use anyhow::{Context, Result, anyhow};
use chrono::Local;
use std::fs;

pub fn append_message(
    config: &Config,
    command_name: &str,
    message: &str,
    as_task: bool,
) -> Result<()> {
    let command = check_command_exists(config, command_name)?;
    let base_directory =
        expand_home(config.base_directory.to_str().unwrap_or("~")).with_context(|| {
            format!(
                "Failed to expand base directory: {:?}",
                config.base_directory
            )
        })?;
    let file_path = conversion_target_file_path(config, command_name, &command)?;
    let now_time = Local::now();

    // Create: Directory and file if they do not exist
    if command.auto_create {
        create_file_if_not_exists(
            &base_directory,
            &file_path,
            &command.template,
            &config.created,
        )?;
    }

    // Open: File
    let contents = fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read file: {:?}", file_path))?;
    let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let set_time = if command.not_format {
        String::new()
    } else {
        now_time
            .format(config.time_format.as_deref().unwrap_or("%H:%M:%S"))
            .to_string()
            + " "
    };
    let in_message = if !as_task {
        format!("- {}{}", set_time, message).to_string()
    } else {
        format!("- [ ] {}{}", set_time, message).to_string()
    };
    // Update: Modified Field
    if let Some(modified_config) = &config.modified
        && let Some(field_name) = &modified_config.field
    {
        update_frontmatter_field(
            &mut lines,
            field_name,
            modified_config
                .format
                .as_deref()
                .unwrap_or("%Y-%m-%d %H:%M:%S"),
        )?;
    }
    // Insert: Message
    if let Some(pattern) = command.insert.as_deref() {
        // `insert` が設定されている場合
        let insert = now_time.format(pattern).to_string();
        let insert_index = find_insert_index(&lines, &insert, command.end_line)?;
        lines.insert(insert_index, in_message);
    } else if command.end_line {
        // `insert` が設定されていないかつ、`end_line=true`
        lines.push(in_message);
    } else {
        // `insert` が設定されていないかつ、`end_line=false`
        let insert_index = lines
            .iter()
            .rposition(|line| line.trim() == "---")
            .map(|index| index + 1)
            .unwrap_or(0);
        lines.insert(insert_index, in_message);
    }

    // Write: File
    let new_contents = lines.join("\n");
    fs::write(&file_path, new_contents)?;
    Ok(())
}

/// `insert`が設定されている場合、挿入位置を返す。
fn find_insert_index(lines: &[String], insert: &str, end_line: bool) -> Result<usize> {
    let anchor = lines
        .iter()
        .rposition(|line| line.trim() == insert)
        .ok_or_else(|| anyhow!("Not Found: Insert line: {:?}", insert))?;
    if !end_line {
        return Ok(anchor + 1);
    }
    Ok(find_block_end(lines, anchor + 1))
}
/// `line`が見出しの行かどうかを判定する。
fn is_heading(line: &str) -> bool {
    let line = line.trim_start();
    line.starts_with('#')
}
/// `line`が箇条書きの行かどうかを判定する。
fn is_list_item(line: &str) -> bool {
    let line = line.trim_start();
    // 箇条書き
    if matches!(line.as_bytes().first(), Some(b'-' | b'*' | b'+')) {
        return line
            .as_bytes()
            .get(1)
            .is_some_and(|c| c.is_ascii_whitespace());
    }
    // 番号付きリスト: 1. xxx / 10) xxx
    let Some(separator) = line.find(['.', ')']) else {
        return false;
    };
    separator > 0
        && line[..separator].chars().all(|c| c.is_ascii_digit())
        && line[separator + 1..]
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_whitespace())
}
/// `line`が空行かどうかを判定する。
fn is_blank(line: &str) -> bool {
    line.trim().is_empty()
}
/// `start`以降にある現在のブロックの終端を返す。
fn find_block_end(lines: &[String], start: usize) -> usize {
    let rest = &lines[start..];
    // 先に次の見出しを探す
    let next_heading = rest
        .iter()
        .position(|line| is_heading(line))
        .map(|index| start + index)
        .unwrap_or(lines.len());

    // 最初の非空行を確認する
    let Some(first_content) = rest.iter().position(|line| !is_blank(line)) else {
        return next_heading;
    };
    let first_content = start + first_content;
    // `insert`の直後がリストでなければ、次の見出しまでをセクションとみなす
    if !is_list_item(&lines[first_content]) {
        return next_heading;
    }
    // リストブロックの終端を探す
    let mut index = first_content;
    while index < next_heading {
        let line = &lines[index];
        if is_list_item(line) {
            index += 1;
            continue;
        }
        if is_blank(line) {
            // 空行の後にリストが続くならリストの一部とみなす
            let next_is_list = lines.get(index + 1).is_some_and(|next| is_list_item(next));
            if next_is_list {
                index += 1;
                continue;
            }
        }
        break;
    }
    index
}
