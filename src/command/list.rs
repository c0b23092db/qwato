use crate::config::{CommandConfig, Config};
use crate::tool::markdown::{is_blank, is_heading, is_list};
use crate::utils::{check_command_exists, conversion_target_directory_path};
use anyhow::{Context, Result};
use chrono::{NaiveDate, NaiveTime};
use colored::*;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct DataLog {
    re: Regex,
    date_stamp: String,
    messages: Vec<MessageLog>,
}
impl DataLog {
    fn new(date_stamp: String, is_all: bool) -> Self {
        Self {
            re: Regex::new(if is_all {
                r"^\s*(?:[-]\s|\d+[.]\s)(?:\[(.)\]\s*)?(?:(\d{2}:\d{2}:\d{2})\s+)?(.*)$"
            } else {
                r"^\s*(?:[-]\s|\d+[.]\s)(?:\[(.)\]\s*)?(\d{2}:\d{2}:\d{2})\s+(.*)$"
            })
            .unwrap(),
            date_stamp,
            messages: Vec::new(),
        }
    }

    fn push(&mut self, line: &str, is_note: bool, is_task: bool) -> bool {
        let Some(captures) = self.re.captures(line) else {
            return false;
        };
        let checkbox = captures.get(1).map(|m| m.as_str());
        let entry_is_task = checkbox.is_some();
        let checked = matches!(checkbox, Some("x") | Some("X"));
        if is_note && entry_is_task {
            return false;
        }
        if is_task && !entry_is_task {
            return false;
        }
        let timestamp = captures
            .get(2)
            .and_then(|m| m.as_str().parse::<NaiveTime>().ok());
        let message = captures.get(3).map_or("", |m| m.as_str()).to_string();
        self.messages.push(MessageLog {
            timestamp,
            kind: if entry_is_task {
                EntryKind::Task { checked }
            } else {
                EntryKind::Memo
            },
            message,
        });
        true
    }
}

#[derive(Debug)]
struct MessageLog {
    timestamp: Option<NaiveTime>,
    kind: EntryKind,
    message: String,
}

#[derive(Debug)]
enum EntryKind {
    Memo,
    Task { checked: bool },
}

/// List: 指定されたコマンドのリスト項目を表示
pub fn list_entries(
    config: &Config,
    command_names: &[String],
    tag_filters: &[String],
    is_note: bool,
    is_task: bool,
    is_all: bool,
) -> Result<()> {
    let mut targets = command_names.to_vec();
    if targets.is_empty() {
        targets.push(
            config
                .default_command
                .clone()
                .expect("Not Setting: default command"),
        );
    }
    let limit = config.list.as_ref().map(|list| list.limit).unwrap_or(10);
    if limit == 0 {
        return Ok(());
    }
    let mut entries: BTreeMap<String, DataLog> = BTreeMap::new();

    // Create: DataLog
    let mut files = Vec::new();
    for command_name in targets {
        let command = check_command_exists(config, &command_name)?;
        let paths = fs::read_dir(conversion_target_directory_path(config, &command)?)
            .with_context(|| format!("Failed to Read Directory: {:?}", command.directory))?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .filter(|path| is_command_file(path, &command))
            .map(|file_path| {
                let date_stamp = resolve_date_stamp(&file_path, &command)?;
                Ok((date_stamp, file_path))
            })
            .collect::<Result<Vec<_>>>()?;
        files.extend(paths);
    }
    files.sort_by(|left, right| right.0.cmp(&left.0));

    for (file_index, (date_stamp, file_path)) in files.iter().enumerate() {
        let contents = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to Read: {}", file_path.display()))?;
        let data_log = entries
            .entry(date_stamp.clone())
            .or_insert_with(|| DataLog::new(date_stamp.clone(), is_all));
        let mut in_frontmatter = false;
        let mut in_code_block = false;
        let mut current_index: Option<usize> = None;
        for line in contents.lines() {
            let trimmed = line.trim();
            // Skip: Frontmatter and Code Block
            if trimmed == "---" {
                in_frontmatter = !in_frontmatter;
                in_code_block = false;
                current_index = None;
                continue;
            }
            if in_frontmatter {
                continue;
            }
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
                current_index = None;
                continue;
            }
            if in_code_block {
                continue;
            }
            if is_blank(trimmed) || is_heading(trimmed) || trimmed.starts_with('>') {
                current_index = None;
                continue;
            }
            if is_list(trimmed) {
                current_index = None;
            }
            if data_log.push(line, is_note, is_task) {
                current_index = Some(data_log.messages.len() - 1);
                continue;
            }
            if let Some(index) = current_index {
                let current_message = &mut data_log.messages[index].message;
                current_message.push('\n');
                current_message.push_str(trimmed);
            }
        }

        let next_date = files.get(file_index + 1).map(|file| file.0.as_str());
        if next_date != Some(date_stamp.as_str())
            && reached_limit(&entries, tag_filters, is_note, is_task, limit)
        {
            break;
        }
    }

    // Sort: 日付順
    let mut sorted_entries: Vec<DataLog> = entries
        .into_values()
        .map(|mut data_log| {
            data_log.messages.retain(|entry| {
                matches!(entry.kind, EntryKind::Memo)
                    || matches!(entry.kind, EntryKind::Task { checked: false })
            });
            data_log
                .messages
                .retain(|entry| message_matches_tags(&entry.message, tag_filters));
            data_log
                .messages
                .sort_by_key(|entry| Reverse(entry.timestamp.unwrap_or(NaiveTime::MIN)));
            data_log
        })
        .collect();
    sorted_entries.sort_by(|left, right| right.date_stamp.cmp(&left.date_stamp));

    let mut printed = 0usize;
    for data_log in sorted_entries {
        if printed >= limit {
            return Ok(());
        }
        if data_log.messages.is_empty() {
            continue;
        }
        println!("{}", data_log.date_stamp.blue().bold());
        for message in data_log.messages {
            if printed >= limit {
                return Ok(());
            }
            match message.kind {
                EntryKind::Memo => {
                    print_message(message.timestamp, None, &message.message);
                }
                EntryKind::Task { checked } => {
                    let checkbox = if checked { "[x]" } else { "[ ]" };
                    print_message(message.timestamp, Some(checkbox), &message.message);
                }
            }
            printed += 1;
        }
    }

    Ok(())
}

/// Check: コマンドのファイル名
fn is_command_file(file_path: &Path, command: &CommandConfig) -> bool {
    let Some(pattern) = command.file.as_deref() else {
        return true;
    };
    let Some(file_name) = file_path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let pattern = pattern.to_string_lossy();
    file_name == pattern || NaiveDate::parse_from_str(file_name, &pattern).is_ok()
}

/// Check: Tag
fn message_matches_tags(message: &str, tag_filters: &[String]) -> bool {
    if tag_filters.is_empty() {
        return true;
    }
    let mut tags = HashSet::new();
    for token in message.split_whitespace() {
        let tag = token.strip_prefix('#').unwrap_or(token).trim();
        if !tag.is_empty() {
            tags.insert(normalize_tag(tag));
        }
    }
    tag_filters
        .iter()
        .map(|tag| normalize_tag(tag))
        .any(|tag| tags.contains(&tag))
}

/// Check: 表示対象数が上限に達したか
fn reached_limit(
    entries: &BTreeMap<String, DataLog>,
    tag_filters: &[String],
    is_note: bool,
    is_task: bool,
    limit: usize,
) -> bool {
    entries
        .values()
        .flat_map(|data_log| data_log.messages.iter())
        .filter(|entry| {
            (is_note && !is_task && matches!(entry.kind, EntryKind::Memo))
                || (is_task && !is_note && matches!(entry.kind, EntryKind::Task { checked: false }))
                || (!is_note
                    && !is_task
                    && (matches!(entry.kind, EntryKind::Memo)
                        || matches!(entry.kind, EntryKind::Task { checked: false })))
        })
        .filter(|entry| message_matches_tags(&entry.message, tag_filters))
        .count()
        >= limit
}

/// Normalize: Tag
fn normalize_tag(tag: &str) -> String {
    tag.trim().trim_matches('#').trim().to_ascii_lowercase()
}

/// Resolve: ファイル名から日付を取得する
fn resolve_date_stamp(file_path: &Path, command: &CommandConfig) -> Result<String> {
    if let Some(file_name) = file_path.file_stem().and_then(|s| s.to_str()) {
        if let Some(pattern) = command.file.as_deref()
            && let Ok(date) = NaiveDate::parse_from_str(file_name, &pattern.to_string_lossy())
        {
            return Ok(date.to_string());
        }

        if let Ok(date) = NaiveDate::parse_from_str(file_name, "%Y-%m-%d") {
            return Ok(date.to_string());
        }
    }

    let metadata = fs::metadata(file_path)
        .with_context(|| format!("Failed to Read Metadata: {}", file_path.display()))?;

    if let Ok(created) = metadata.created() {
        let datetime: chrono::DateTime<chrono::Local> = created.into();
        return Ok(datetime.date_naive().to_string());
    }
    if let Ok(modified) = metadata.modified() {
        let datetime: chrono::DateTime<chrono::Local> = modified.into();
        return Ok(datetime.date_naive().to_string());
    }

    Err(anyhow::anyhow!(
        "Failed to resolve date stamp: {:?}",
        file_path
    ))
}

/// Print: メッセージを表示
fn print_message(timestamp: Option<NaiveTime>, checkbox: Option<&str>, message: &str) {
    let mut lines = message.lines();
    let Some(first_line) = lines.next() else {
        return;
    };
    match (timestamp, checkbox) {
        (Some(timestamp), Some(checkbox)) => {
            println!("\t{}\t{} {}", timestamp, checkbox, first_line);
        }
        (Some(timestamp), None) => {
            println!("\t{}\t{}", timestamp, first_line);
        }
        (None, Some(checkbox)) => {
            println!("\t{} {}", checkbox, first_line);
        }
        (None, None) => {
            println!("\t{}", first_line);
        }
    }
    for continuation_line in lines {
        println!("\t\t\t{}", continuation_line);
    }
}
