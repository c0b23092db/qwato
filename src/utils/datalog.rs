use crate::config::{CommandConfig, Config, DailyFile};
use crate::utils::markdown::{is_blank, is_heading, is_list};
use crate::utils::{
    check_command_exists, conversion_target_directory_path, conversion_target_file_path,
};
use anyhow::{Context, Result, anyhow};
use chrono::{NaiveDate, NaiveTime};
use regex::Regex;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageLog {
    pub timestamp: Option<NaiveTime>,
    pub kind: EntryKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    Memo,
    Task { checked: bool },
}

#[derive(Debug, Clone)]
pub struct DataLog {
    pub re: Regex,
    pub date_stamp: String,
    pub messages: Vec<MessageLog>,
}

impl DataLog {
    pub fn new(date_stamp: String, is_all: bool) -> Self {
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

    pub fn push(&mut self, line: &str, is_note: bool, is_task: bool) -> bool {
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

    /// ファイルの内容をパースしてDataLogを生成
    pub fn parse(
        contents: &str,
        date_stamp: &str,
        is_note: bool,
        is_task: bool,
        is_all: bool,
    ) -> Self {
        let mut data_log = Self::new(date_stamp.to_string(), is_all);
        let mut in_frontmatter = false;
        let mut in_code_block = false;
        let mut current_index: Option<usize> = None;

        for line in contents.lines() {
            let trimmed = line.trim();

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

        data_log
    }

    /// Header形式の複数日付セクションを持つファイルをパースし、entriesに追加する
    pub fn parse_header_contents(
        entries: &mut BTreeMap<String, DataLog>,
        contents: &str,
        default_date: Option<&str>,
        insert_pattern: Option<&str>,
        is_note: bool,
        is_task: bool,
        is_all: bool,
    ) {
        let mut in_frontmatter = false;
        let mut in_code_block = false;
        let mut current_date: Option<String> = default_date.map(ToString::to_string);
        let mut current_index: Option<usize> = None;

        for line in contents.lines() {
            let trimmed = line.trim();

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
            if is_blank(trimmed) || trimmed.starts_with('>') {
                current_index = None;
                continue;
            }

            if is_heading(trimmed) {
                current_index = None;
                if let Some(date_stamp) = parse_heading_date(trimmed, insert_pattern) {
                    current_date = Some(date_stamp);
                }
                continue;
            }

            if is_list(trimmed) {
                current_index = None;
            }

            if let Some(ref date_stamp) = current_date {
                let data_log = entries
                    .entry(date_stamp.clone())
                    .or_insert_with(|| DataLog::new(date_stamp.clone(), is_all));
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
        }
    }

    pub fn sort_messages(&mut self) {
        self.messages
            .sort_by_key(|entry| Reverse(entry.timestamp.unwrap_or(NaiveTime::MIN)));
    }

    /// 複数コマンドからDataLogを収集して日付ごとのマップを生成
    pub fn collect_from_commands(
        config: &Config,
        command_names: &[String],
        is_note: bool,
        is_task: bool,
        is_all: bool,
    ) -> Result<BTreeMap<String, DataLog>> {
        let mut targets = command_names.to_vec();
        if targets.is_empty() {
            targets.push(
                config
                    .default_command
                    .clone()
                    .context("Not Setting: default command")?,
            );
        }

        let mut entries = BTreeMap::new();
        for command_name in targets {
            let command_entries =
                Self::collect_from_command(config, &command_name, is_note, is_task, is_all)?;
            for (date, data_log) in command_entries {
                let entry = entries
                    .entry(date.clone())
                    .or_insert_with(|| DataLog::new(date, is_all));
                entry.messages.extend(data_log.messages);
            }
        }
        Ok(entries)
    }

    /// 単一コマンドからDataLogを収集して日付ごとのマップを生成
    pub fn collect_from_command(
        config: &Config,
        command_name: &str,
        is_note: bool,
        is_task: bool,
        is_all: bool,
    ) -> Result<BTreeMap<String, DataLog>> {
        let command = check_command_exists(config, command_name)?;
        let dir_path = conversion_target_directory_path(config, &command)?;
        let files = collect_files(&dir_path)
            .with_context(|| format!("Failed to Read Directory: {:?}", command.directory))?
            .into_iter()
            .filter(|path| is_command_file(path, &command))
            .collect::<Vec<_>>();

        let mut entries = BTreeMap::new();

        // Check if file path contains date placeholders
        let has_date_placeholder = command
            .file
            .as_ref()
            .map(|f| {
                let file_str = f.to_string_lossy();
                file_str.contains('%')
            })
            .unwrap_or(true);

        if matches!(command.date_format(), DailyFile::Header) {
            // Get: ヘッダ形式のファイル //
            for file_path in files {
                let contents = fs::read_to_string(&file_path)
                    .with_context(|| format!("Failed to Read: {}", file_path.display()))?;
                Self::parse_header_contents(
                    &mut entries,
                    &contents,
                    None,
                    command.insert.as_deref(),
                    is_note,
                    is_task,
                    is_all,
                );
            }
        } else if matches!(command.date_format(), DailyFile::Onefile) || !has_date_placeholder {
            // Get: 一つのファイル //
            let file_path = conversion_target_file_path(config, command_name, &command)?;
            let contents = fs::read_to_string(&file_path)
                .with_context(|| format!("Failed to Read: {}", file_path.display()))?;
            let date_stamp = resolve_date_stamp(&file_path, &command)?;
            let parsed = DataLog::parse(&contents, &date_stamp, is_note, is_task, is_all);
            let data_log = entries
                .entry(date_stamp.clone())
                .or_insert_with(|| DataLog::new(date_stamp, is_all));
            data_log.messages.extend(parsed.messages);
        } else {
            // Get: 複数ファイル //
            let mut stamped_files = Vec::new();
            for file_path in files {
                let date_stamp = resolve_date_stamp(&file_path, &command)?;
                stamped_files.push((date_stamp, file_path));
            }
            stamped_files.sort_by(|left, right| right.0.cmp(&left.0));

            for (date_stamp, file_path) in stamped_files {
                let contents = fs::read_to_string(&file_path)
                    .with_context(|| format!("Failed to Read: {}", file_path.display()))?;
                let parsed = DataLog::parse(&contents, &date_stamp, is_note, is_task, is_all);
                let data_log = entries
                    .entry(date_stamp.clone())
                    .or_insert_with(|| DataLog::new(date_stamp, is_all));
                data_log.messages.extend(parsed.messages);
            }
        }

        Ok(entries)
    }
}

/// ディレクトリ内の全ファイルを再帰的に探索
pub fn collect_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if !dir.exists() {
        return Ok(files);
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(collect_files(&path)?);
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(files)
}

/// コマンドのファイル名に合致するか判定
pub fn is_command_file(file_path: &Path, command: &CommandConfig) -> bool {
    let Some(pattern) = command.file.as_deref() else {
        return true;
    };
    let pattern_str = pattern.to_string_lossy();
    let file_name = file_path.file_name().and_then(|name| name.to_str());

    if file_name == Some(pattern_str.as_ref()) {
        return true;
    }

    let format = command.date_format();
    match format.parse_date_from_path(file_path) {
        Some(_) => true,
        None => file_name
            .map(|name| NaiveDate::parse_from_str(name, &pattern_str).is_ok())
            .unwrap_or(false),
    }
}

/// 見出し行から日付を抽出
fn parse_heading_date(heading: &str, insert_pattern: Option<&str>) -> Option<String> {
    let trimmed = heading.trim();
    if !is_heading(trimmed) {
        return None;
    }
    let content = trimmed.trim_start_matches('#').trim();

    if let Some(pattern) = insert_pattern {
        let clean_pattern = pattern.trim_start_matches('#').trim();
        if let Ok(date) = NaiveDate::parse_from_str(content, clean_pattern) {
            return Some(date.to_string());
        }
    }

    if let Ok(date) = NaiveDate::parse_from_str(content, "%Y-%m-%d") {
        return Some(date.to_string());
    }
    if let Ok(date) = NaiveDate::parse_from_str(content, "%Y/%m/%d") {
        return Some(date.to_string());
    }
    None
}

/// ファイルパスから日付スタンプを取得
pub fn resolve_date_stamp(file_path: &Path, command: &CommandConfig) -> Result<String> {
    let format = command.date_format();
    if let Some(date) = format.parse_date_from_path(file_path) {
        return Ok(date.to_string());
    }

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

    Err(anyhow!("Failed to resolve date stamp: {:?}", file_path))
}
