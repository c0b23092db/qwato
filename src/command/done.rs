// Bug: Option `--done` //

use crate::tool::datalog::DataLog;

pub fn mark_task_done(
    config: &Config,
    command_names: &[String],
    target_index: usize,
    tag_filters: &[String],
) -> Result<()> {
    let mut tasks = collect_task_entries(config, command_names, tag_filters)?;
    tasks.retain(|task| task.is_unchecked());
    tasks.sort_by(|left, right| {
        let by_date = right.date_stamp.cmp(&left.date_stamp);
        if !by_date.is_eq() {
            return by_date;
        }
        let left_time = left.timestamp.unwrap_or(NaiveTime::MIN);
        let right_time = right.timestamp.unwrap_or(NaiveTime::MIN);
        right_time.cmp(&left_time)
    });

    let Some(task) = pick_task_by_index(&tasks, target_index) else {
        return Err(anyhow!(
            "No task found for index {} in the current filtered list",
            target_index
        ));
    };

    let mut lines: Vec<String> = fs::read_to_string(&task.file_path)
        .with_context(|| format!("Failed to Read File: {:?}", task.file_path))?
        .lines()
        .map(std::string::ToString::to_string)
        .collect();

    let target_line = &mut lines[task.line_number];
    if target_line.contains("[ ]") {
        *target_line = target_line.replace("[ ]", "[x]");
    } else {
        return Ok(());
    }

    let new_contents = lines.join("\n");
    fs::write(&task.file_path, new_contents)
        .with_context(|| format!("Failed to update task file: {:?}", task.file_path))?;
    Ok(())
}

/// Collect: タスクのリストを収集
fn collect_task_entries(
    config: &Config,
    command_names: &[String],
    tag_filters: &[String],
) -> Result<Vec<TaskEntry>> {
    let mut targets = command_names.to_vec();
    if targets.is_empty() {
        targets.push(
            config
                .default_command
                .clone()
                .with_context(|| "Not Setting: default command")?,
        );
    }

    let mut tasks = Vec::new();
    for command_name in targets {
        let command = check_command_exists(config, &command_name)?;
        let paths = fs::read_dir(conversion_target_directory_path(config, &command)?)
            .with_context(|| format!("Failed to read directory: {:?}", command.directory))?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .collect::<Vec<_>>();

        for file_path in paths {
            let contents = fs::read_to_string(&file_path)
                .with_context(|| format!("Failed to read file: {:?}", file_path))?;
            let date_stamp = resolve_date_stamp(&file_path, &command)?;
            let mut in_frontmatter = false;
            let mut in_code_block = false;

            for (line_number, line) in contents.lines().enumerate() {
                let trimmed = line.trim();
                if trimmed == "---" {
                    in_frontmatter = !in_frontmatter;
                    in_code_block = false;
                    continue;
                }
                if in_frontmatter || in_code_block {
                    continue;
                }
                if trimmed.starts_with("```") {
                    in_code_block = !in_code_block;
                    continue;
                }
                if is_blank(trimmed) || is_heading(trimmed) || trimmed.starts_with('>') {
                    continue;
                }

                let Some(captures) = DataLog::new(date_stamp.clone(), false).re.captures(line)
                else {
                    continue;
                };
                let checkbox = captures.get(1).map(|m| m.as_str());
                if checkbox.is_none() {
                    continue;
                }
                let message = captures.get(3).map_or("", |m| m.as_str()).to_string();
                if !message_matches_tags(&message, tag_filters) {
                    continue;
                }
                let timestamp = captures
                    .get(2)
                    .and_then(|m| m.as_str().parse::<NaiveTime>().ok());
                tasks.push(TaskEntry {
                    date_stamp: date_stamp.clone(),
                    file_path: file_path.clone(),
                    line_number,
                    timestamp,
                });
            }
        }
    }

    Ok(tasks)
}

#[derive(Debug, Clone)]
struct TaskEntry {
    date_stamp: String,
    file_path: PathBuf,
    line_number: usize,
    timestamp: Option<NaiveTime>,
}

impl TaskEntry {
    fn is_unchecked(&self) -> bool {
        let file = fs::read_to_string(&self.file_path).ok();
        let Some(file) = file else {
            return false;
        };
        let Some(line) = file.lines().nth(self.line_number) else {
            return false;
        };

        let Some(captures) = DataLog::new(String::new(), false).re.captures(line) else {
            return false;
        };
        captures
            .get(1)
            .map(|m| !matches!(m.as_str(), "x" | "X"))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::{TaskEntry, message_matches_tags, pick_task_by_index};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn message_matches_tags_accepts_any_matching_tag() {
        assert!(message_matches_tags(
            "- 10:12:34 #Software Obsidian",
            &["Software".to_string(), "daily".to_string()],
        ));
        assert!(message_matches_tags(
            "- 12:23:45 #Game Minecraft",
            &["Game".to_string(), "daily".to_string()],
        ));
        assert!(!message_matches_tags(
            "- 17:00:00 #Game Minecraft",
            &["Software".to_string(), "daily".to_string()],
        ));
    }

    #[test]
    fn pick_task_by_index_uses_newest_first_order() {
        let tasks = vec![
            ("2024-06-20", 3, "- [ ] 12:00:00 #daily third"),
            ("2024-06-20", 2, "- [ ] 11:00:00 #software second"),
            ("2024-06-20", 1, "- [ ] 10:00:00 #daily first"),
        ];

        let picked = pick_task_by_index(&tasks, 2).unwrap();
        assert_eq!(picked.2, "- [ ] 11:00:00 #software second");
    }

    #[test]
    fn unchecked_task_filter_ignores_done_entries() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("qwato-task-test-{unique}"));
        fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("2026-08-19.md");
        fs::write(
            &file_path,
            "- [ ] 11:29:26 Test3\n- [x] 11:29:25 Test2\n- [ ] 11:29:23 Test1\n",
        )
        .unwrap();

        let unchecked = TaskEntry {
            date_stamp: "2026-08-19".to_string(),
            file_path: file_path.clone(),
            line_number: 1,
            timestamp: None,
        };
        let open = TaskEntry {
            date_stamp: "2026-08-19".to_string(),
            file_path,
            line_number: 0,
            timestamp: None,
        };

        assert!(!unchecked.is_unchecked());
        assert!(open.is_unchecked());
    }

    #[test]
    fn data_log_accepts_tasks_without_timestamps() {
        let mut data_log = super::DataLog::new("2026-08-19".to_string(), false);

        assert!(data_log.push("- [ ] Test3", false, true));
        assert_eq!(data_log.messages.len(), 1);
        assert_eq!(data_log.messages[0].timestamp, None);
        assert_eq!(data_log.messages[0].message, "Test3");
    }
}

/// Pick: タスクをインデックスで選択
pub fn pick_task_by_index<T: Clone>(tasks: &[T], index: usize) -> Option<T> {
    let target_index = index.saturating_sub(1);
    tasks.get(target_index).cloned()
}
