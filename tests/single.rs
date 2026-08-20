mod utils;
use utils::{assert_success, parse_datetime_from_output, read_output, remove_output, run_qwa};

const ADD_CONFIG: &str = "./tests/config/add.toml";
const CHECKBOX_CONFIG: &str = "./tests/config/checkbox.toml";

#[test]
fn single_add() {
    let expected_path = "./tests/output/add/single_add.md";
    remove_output(expected_path);
    let output = run_qwa(
        ADD_CONFIG,
        &[
            "--add",
            "single_add",
            "This is a test message",
            "--utc-offset-time",
        ],
    );
    assert_success(&output);
    let now_time = parse_datetime_from_output(&output, "%H:%M:%S");
    let actual = format!("- {now_time} This is a test message");
    let expected = read_output(expected_path);
    assert_eq!(actual, expected);
}

#[test]
fn single_add_not_format() {
    let expected_path = "./tests/output/add/single_add_not_format.md";
    remove_output(expected_path);
    let output = run_qwa(
        ADD_CONFIG,
        &["--add", "single_add_not_format", "This is a test message"],
    );
    assert_success(&output);
    let actual = "- This is a test message";
    let expected = read_output(expected_path);
    assert_eq!(actual, expected);
}

#[test]
fn single_checkbox() {
    let expected_path = "./tests/output/check/single_checkbox.md";
    remove_output(expected_path);
    let output = run_qwa(
        CHECKBOX_CONFIG,
        &[
            "--checkbox",
            "--utc-offset-time",
            "single_checkbox",
            "This is a test message",
        ],
    );
    assert_success(&output);
    let now_time = parse_datetime_from_output(&output, "%H:%M:%S");
    let actual = format!("- [ ] {now_time} This is a test message");
    let expected = read_output(expected_path);
    assert_eq!(actual, expected);
}

#[test]
fn single_checkbox_not_format() {
    let expected_path = "./tests/output/check/single_checkbox_not_format.md";
    remove_output(expected_path);
    let output = run_qwa(
        CHECKBOX_CONFIG,
        &[
            "--checkbox",
            "single_checkbox_not_format",
            "This is a test message",
        ],
    );
    assert_success(&output);
    let actual = "- [ ] This is a test message";
    let expected = read_output(expected_path);
    assert_eq!(actual, expected);
}
