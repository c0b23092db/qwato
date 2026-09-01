use crate::config::Config;
use crate::utils::datalog::{DataLog, EntryKind};
use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use colored::*;
use std::collections::HashSet;

/// List: 指定されたコマンドのリスト項目を表示
#[allow(clippy::too_many_arguments)]
pub fn list_entries(
    config: &Config,
    command_names: &[String],
    tag_filters: &[String],
    is_note: bool,
    is_task: bool,
    is_all: bool,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
) -> Result<()> {
    let limit = config.list.as_ref().map(|list| list.limit).unwrap_or(10);
    if limit == 0 {
        return Ok(());
    }

    let entries = DataLog::collect_from_commands(config, command_names, is_note, is_task, is_all)?;

    // Sort: 日付順
    let mut sorted_entries: Vec<DataLog> = entries
        .into_values()
        .filter(|data_log| {
            if let Ok(date) = NaiveDate::parse_from_str(&data_log.date_stamp, "%Y-%m-%d") {
                if let Some(from) = from_date
                    && date < from
                {
                    return false;
                }
                if let Some(to) = to_date
                    && date > to
                {
                    return false;
                }
            }
            true
        })
        .map(|mut data_log| {
            data_log.messages.retain(|entry| {
                matches!(entry.kind, EntryKind::Memo)
                    || matches!(entry.kind, EntryKind::Task { checked: false })
            });
            data_log
                .messages
                .retain(|entry| message_matches_tags(&entry.message, tag_filters));
            data_log.sort_messages();
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
                EntryKind::Memo => print_message(message.timestamp, &message.message, None),
                EntryKind::Task { checked } => {
                    print_message(message.timestamp, &message.message, Some(checked))
                }
            }
            printed += 1;
        }
    }

    Ok(())
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

/// Normalize: Tag
fn normalize_tag(tag: &str) -> String {
    tag.trim().trim_matches('#').trim().to_ascii_lowercase()
}

/// Print: メッセージを表示
fn print_message(timestamp: Option<NaiveTime>, message: &str, checkbox: Option<bool>) {
    let mut lines = message.lines();
    let Some(first_line) = lines.next() else {
        return;
    };
    match (timestamp, checkbox) {
        (Some(timestamp), Some(checkbox)) => {
            let checkbox_str = if checkbox { "[x]" } else { "[ ]" };
            println!("\t{}\t{} {}", timestamp, checkbox_str, first_line);
        }
        (Some(timestamp), None) => {
            println!("\t{}\t{}", timestamp, first_line);
        }
        (None, Some(checkbox)) => {
            let checkbox_str = if checkbox { "[x]" } else { "[ ]" };
            println!("\t{} {}", checkbox_str, first_line);
        }
        (None, None) => {
            println!("\t{}", first_line);
        }
    }
    for continuation_line in lines {
        println!("\t\t\t{}", continuation_line);
    }
}
