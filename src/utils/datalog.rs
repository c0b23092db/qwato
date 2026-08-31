use crate::utils::markdown::{is_blank, is_heading, is_list};
use chrono::NaiveTime;
use regex::Regex;
use std::cmp::Reverse;

#[derive(Debug, Clone)]
pub struct DataLog {
    re: Regex,
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

    pub fn parse(contents: &str, date_stamp: &str, is_note: bool, is_task: bool, is_all: bool) -> Self {
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

    pub fn sort_messages(&mut self) {
        self.messages.sort_by_key(|entry| Reverse(entry.timestamp.unwrap_or(NaiveTime::MIN)));
    }
}

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

#[cfg(test)]
mod tests {
    use super::DataLog;

    #[test]
    fn parse_task_entry_without_timestamp_requires_all_mode() {
        let mut data_log = DataLog::new("2026-08-19".to_string(), false);
        assert!(!data_log.push("- [ ] Test3", false, true));

        let mut all_data_log = DataLog::new("2026-08-19".to_string(), true);
        assert!(all_data_log.push("- [ ] Test3", false, true));
        assert_eq!(all_data_log.messages.len(), 1);
        assert_eq!(all_data_log.messages[0].timestamp, None);
        assert_eq!(all_data_log.messages[0].message, "Test3");
    }

    #[test]
    fn parse_memo_entry_with_timestamp() {
        let mut data_log = DataLog::new("2026-08-19".to_string(), false);
        assert!(data_log.push("- 12:34:56 memo text", false, false));
        assert_eq!(data_log.messages.len(), 1);
        assert_eq!(data_log.messages[0].timestamp.unwrap().to_string(), "12:34:56");
        assert_eq!(data_log.messages[0].message, "memo text");
    }
}
