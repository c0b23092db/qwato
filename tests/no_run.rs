mod utils;
use utils::{remove_output, run_qwa};

const CONFIG: &str = "./tests/config/non.toml";
const MESSAGE: &str = "This is a test message";

fn assert_error(config_path: &str, args: &[&str], expected: &str) {
    let output = run_qwa(config_path, args);
    assert!(!output.status.success());
    assert_eq!(expected, String::from_utf8_lossy(&output.stderr).trim());
}

#[test]
fn no_message() {
    assert_error(CONFIG, &["--add"], "No message provided");
}

#[test]
fn no_default_command() {
    assert_error(CONFIG, &["--add", MESSAGE], "Not Config: defualt_command");
}

#[test]
fn no_command() {
    assert_error(
        CONFIG,
        &["--add", "no_command", MESSAGE],
        "Unknown Command: no_command",
    );
}

#[test]
fn no_config() {
    assert_error(
        "./tests/config/no_config.toml",
        &["--add", "no_command", MESSAGE],
        "Failed to Find: ./tests/config/no_config.toml",
    );
}

#[test]
fn no_auto_create() {
    assert_error(
        CONFIG,
        &["--add", "auto_create_false", MESSAGE],
        "Failed to Read: ./tests/output\\auto_create.md",
    );
}

#[test]
fn no_template() {
    assert_error(
        CONFIG,
        &["--add", "no_template", MESSAGE],
        "Not Exist Template File: ./tests/output\\../template/no_template.md",
    );
}

#[test]
fn no_created() {
    let expected_path = "./tests/output/non/not_created.md";
    remove_output(expected_path);
    assert_error(
        CONFIG,
        &["--add", "not_created", MESSAGE],
        "Failed to Found Frontmatter: Created",
    );
}

#[test]
fn no_modified() {
    let expected_path = "./tests/output/not/not_created.md";
    remove_output(expected_path);
    assert_error(
        CONFIG,
        &["--add", "not_modified", MESSAGE],
        "Failed to Found Frontmatter: Modified",
    );
}
