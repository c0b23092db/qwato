mod utils;
use utils::{remove_output, run_qwa};

const CONFIG: &str = "./tests/config/add.toml";

#[test]
fn test_summary_command() {
    let target_file = "./tests/output/add/single_add.md";
    remove_output(target_file);

    // Create / Update file today
    let output_add = run_qwa(CONFIG, &["--add", "single_add", "summary test memo"]);
    assert!(output_add.status.success());

    // Run summary
    let output_summary = run_qwa(CONFIG, &["--summary"]);
    assert!(
        output_summary.status.success(),
        "Failed summary: {}",
        String::from_utf8_lossy(&output_summary.stderr)
    );

    let stdout = String::from_utf8(output_summary.stdout).expect("output was not utf-8");
    let lines: Vec<&str> = stdout.lines().collect();

    // The created/modified entry should be listed in summary output
    assert!(
        lines.iter().any(|line| line.contains("summary test memo")),
        "Expected 'summary test memo' in summary output, got: {:?}",
        lines
    );
}

#[test]
fn test_summary_with_specific_command() {
    let target_file = "./tests/output/add/add_format_tag.md";
    remove_output(target_file);

    let output_add = run_qwa(CONFIG, &["--add", "add_format_tag", "test memo"]);
    assert!(output_add.status.success());

    let output_summary = run_qwa(CONFIG, &["--summary", "add_format_tag"]);
    assert!(output_summary.status.success());

    let stdout = String::from_utf8(output_summary.stdout).expect("output was not utf-8");
    let lines: Vec<&str> = stdout.lines().collect();

    assert!(
        lines.iter().any(|line| line.contains("test memo")),
        "Expected 'test memo' in summary output, got: {:?}",
        lines
    );
}
