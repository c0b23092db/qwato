mod utils;
use utils::{read_output, remove_output, run_qwa};

const CONFIG: &str = "./tests/config/add.toml";

#[test]
fn test_last_edit_default() {
    let target_file = "./tests/output/add/single_add.md";
    remove_output(target_file);

    // Add first memo
    let output1 = run_qwa(CONFIG, &["--add", "single_add", "first memo"]);
    assert!(output1.status.success());

    // Add second memo
    let output2 = run_qwa(CONFIG, &["--add", "single_add", "second memo"]);
    assert!(output2.status.success());

    // Check before edit
    let content = read_output(target_file);
    assert!(content.contains("second memo"));
    assert!(content.contains("first memo"));

    // Replace last memo with --last-edit
    let output_edit = run_qwa(CONFIG, &["single_add", "--last-edit", "aaa"]);
    assert!(
        output_edit.status.success(),
        "Failed last-edit: {}",
        String::from_utf8_lossy(&output_edit.stderr)
    );

    let content_after = read_output(target_file);
    assert!(content_after.contains("aaa"));
    assert!(!content_after.contains("second memo"));
    assert!(content_after.contains("first memo"));

    // Verify timestamp was preserved
    let lines: Vec<&str> = content_after.lines().collect();
    let edited_line = lines.iter().find(|l| l.contains("aaa")).unwrap();
    assert!(edited_line.starts_with("- "));
    // Format is "- HH:MM:SS aaa"
    assert_eq!(edited_line.split_whitespace().last(), Some("aaa"));
}

#[test]
fn test_last_edit_with_insert() {
    let target_file = "./tests/output/add/add_some_insert.md";
    remove_output(target_file);

    let output1 = run_qwa(CONFIG, &["--add", "add_some_insert", "section memo 1"]);
    assert!(output1.status.success());

    let output2 = run_qwa(CONFIG, &["--add", "add_some_insert", "section memo 2"]);
    assert!(output2.status.success());

    let output_edit = run_qwa(
        CONFIG,
        &["add_some_insert", "--last-edit", "section memo updated"],
    );
    assert!(output_edit.status.success());

    let content = read_output(target_file);
    assert!(content.contains("section memo updated"));
    assert!(!content.contains("section memo 2"));
    assert!(content.contains("section memo 1"));
}
