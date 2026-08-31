mod utils;
use utils::run_qwa;

const CONFIG: &str = "./tests/config/list.toml";

fn run_qwa_cmd(args: &[&str]) -> String {
    let output = run_qwa(CONFIG, args);
    assert!(
        output.status.success(),
        "qwa failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("qwa output was not UTF-8")
}

fn output_lines(output: &str) -> Vec<&str> {
    output.lines().collect()
}

#[test]
fn note_date_range_from_to() {
    let output = run_qwa_cmd(&[
        "--note",
        "--from",
        "2025-07-27",
        "--to",
        "2025-07-28",
        "--limit",
        "20",
    ]);
    let lines = output_lines(&output);
    assert!(lines.contains(&"2025-07-28"));
    assert!(lines.contains(&"2025-07-27"));
    assert!(!lines.contains(&"2025-07-29"));
    assert!(!lines.contains(&"2025-07-26"));
    assert!(!lines.contains(&"2025-07-25"));
}

#[test]
fn note_date_range_from_only() {
    let output = run_qwa_cmd(&["--note", "--from", "2025-07-28", "--limit", "20"]);
    let lines = output_lines(&output);
    assert!(lines.contains(&"2025-07-29"));
    assert!(lines.contains(&"2025-07-28"));
    assert!(!lines.contains(&"2025-07-27"));
    assert!(!lines.contains(&"2025-07-26"));
    assert!(!lines.contains(&"2025-07-25"));
}

#[test]
fn note_date_range_to_only() {
    let output = run_qwa_cmd(&["--note", "--to", "2025-07-26", "--limit", "20"]);
    let lines = output_lines(&output);
    assert!(!lines.contains(&"2025-07-29"));
    assert!(!lines.contains(&"2025-07-28"));
    assert!(!lines.contains(&"2025-07-27"));
    assert!(lines.contains(&"2025-07-26"));
    assert!(lines.contains(&"2025-07-25"));
}

#[test]
fn list_date_range_with_slash_format() {
    let output = run_qwa_cmd(&[
        "--list",
        "--from",
        "2025/07/27",
        "--to",
        "2025/07/28",
        "--limit",
        "20",
    ]);
    let lines = output_lines(&output);
    assert!(lines.contains(&"2025-07-28"));
    assert!(lines.contains(&"2025-07-27"));
    assert!(!lines.contains(&"2025-07-29"));
}
