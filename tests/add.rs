use chrono::DateTime;

mod utils;
use utils::{assert_success, parse_datetime_from_output, read_output, remove_output, run_qwa};

const CONFIG: &str = "./tests/config/add.toml";

#[test]
fn add_end_line() {
    let expected_path = "./tests/output/add/add_end_line.md";
    remove_output(expected_path);
    let mut expected = String::new();
    for count in 0..10 {
        let message = format!("This is a test message {}", count);
        let command = if count % 2 == 0 {
            "add_end_line_true"
        } else {
            "add_end_line_false"
        };
        let output = run_qwa(CONFIG, &["--add", command, &message]);
        assert_success(&output);
        if count % 2 == 0 {
            if count != 0 {
                expected.push('\n');
            }
            expected.push_str(&format!("- {message}"));
        } else {
            expected = format!("- {message}\n") + &expected;
        }
        assert_eq!(read_output(expected_path), expected);
    }
}

#[test]
fn add_tag() {
    let expected_path = "./tests/output/add/add_tag.md";
    remove_output(expected_path);
    let output = run_qwa(
        CONFIG,
        &[
            "--add",
            "--tag",
            "tag0,tag1,tag2",
            "--",
            "add_tag",
            "This is a test message",
        ],
    );
    assert_success(&output);
    let actual = read_output(expected_path);
    assert_eq!(actual, "- #tag0 #tag1 #tag2 This is a test message");
}

#[test]
fn add_format_tag() {
    let expected_path = "./tests/output/add/add_format_tag.md";
    remove_output(expected_path);
    remove_output(expected_path);
    let output = run_qwa(
        CONFIG,
        &[
            "--add",
            "--tag",
            "tag0,tag1,tag2",
            "--utc-offset-time",
            "add_format_tag",
            "This is a test message",
        ],
    );
    assert_success(&output);
    let expected = read_output(expected_path);
    let now_time = parse_datetime_from_output(&output, "%H:%M:%S");
    let actual = format!("- {now_time} #tag0 #tag1 #tag2 This is a test message");
    assert_eq!(actual, expected);
}

#[test]
fn add_config_tag() {
    let expected_path = "./tests/output/add/add_config_tag.md";
    remove_output(expected_path);
    let output = run_qwa(
        CONFIG,
        &["--add", "add_config_tag", "This is a test message"],
    );
    assert_success(&output);
    let actual = read_output(expected_path);
    assert_eq!(actual, "- #config #tag This is a test message");
}

#[test]
fn add_insert_end_line() {
    let expected_path = "./tests/output/add/add_insert_end_line.md";
    let insert = "## Add";
    remove_output(expected_path);
    for count in 0..10 {
        let message = format!("This is a test message {}", count);
        let command = if count % 2 == 0 {
            "add_insert_end_line_true"
        } else {
            "add_insert_end_line_false"
        };
        let output = run_qwa(CONFIG, &["--add", command, &message]);
        assert_success(&output);
        let contents = read_output(expected_path);
        let actual = contents
            .lines()
            .skip_while(|line| line.trim() != insert)
            .skip(1)
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>();
        let mut expected = Vec::new();
        for index in 0..=count {
            let message = format!("- This is a test message {index}");
            if index % 2 == 0 {
                expected.push(message);
            } else {
                expected.insert(0, message);
            }
        }
        assert_eq!(
            actual,
            expected.iter().map(String::as_str).collect::<Vec<_>>()
        );
    }
}

#[test]
fn add_some_insert() {
    let expected_path = "./tests/output/add/add_some_insert.md";
    let insert = "## Add";
    remove_output(expected_path);
    for count in 0..10 {
        let message = format!("This is a test message {}", count);
        let output = run_qwa(CONFIG, &["--add", "add_some_insert", &message]);
        assert_success(&output);
        let contents = read_output(expected_path);
        let lines: Vec<&str> = contents.lines().collect();
        let add_index = lines
            .iter()
            .rposition(|line| line.trim() == insert)
            .expect("Add heading was not found");
        let next_heading = lines
            .iter()
            .enumerate()
            .skip(add_index + 1)
            .find(|(_, line)| line.trim_start().starts_with("#"))
            .map(|(index, _)| index)
            .unwrap_or(lines.len());
        let add_block = &lines[add_index + 1..next_heading];
        let actual: Vec<&str> = add_block
            .iter()
            .copied()
            .filter(|line| !line.trim().is_empty())
            .collect();
        let expected_messages: Vec<String> = (0..=count)
            .map(|index| format!("- This is a test message {index}"))
            .collect();
        let expected: Vec<&str> = expected_messages.iter().map(String::as_str).collect();
        assert!(actual.ends_with(&expected));
    }
}

#[test]
fn add_yaml() {
    let expected_path = "./tests/output/add/add_yaml.md";
    remove_output(expected_path);
    let output = run_qwa(
        CONFIG,
        &[
            "--add",
            "--utc-offset-time",
            "add_yaml",
            "This is a test message",
        ],
    );
    assert_success(&output);
    let datetime = DateTime::parse_from_str(
        String::from_utf8_lossy(&output.stdout).trim(),
        "%Y-%m-%d %H:%M:%S%.f %:z",
    )
    .expect("Failed to Parse: Time");
    let now_data = datetime.format("%Y-%m-%d").to_string();
    let now_time = datetime.format("%H:%M:%S").to_string();
    let actual = format!(
        "---\nCreated: {now_data} {now_time}\nModified: {now_data} {now_time}\n---\n- {now_time} This is a test message"
    );
    let expected = read_output(expected_path);
    assert_eq!(actual, expected);
}
