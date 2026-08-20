mod utils;
use utils::run_qwa;

const CONFIG: &str = "./tests/config/list.toml";

fn run_task(args: &[&str]) -> String {
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
fn normal_task() {
    let output = run_task(&["--task", "--limit", "25"]);
    let lines = output_lines(&output);

    assert_eq!(lines.first(), Some(&"2025-07-29"));
    assert_eq!(lines.last(), Some(&"\t13:45:00\t[ ] 0725.3"));
    assert!(
        lines
            .windows(2)
            .any(|pair| pair == ["2025-07-28", "\t21:19:25\t[ ] 0728.5"])
    );
    assert!(lines.contains(&"\t09:32:16\t[ ] #tag2 0727.1"));
    assert!(!lines.iter().any(|line| line.contains("0729.5")));
}

#[test]
fn task_tag1() {
    let output = run_task(&["--task", "--tag", "tag1", "--limit", "10"]);
    let lines = output_lines(&output);
    assert_eq!(lines, vec!["2025-07-26", "\t15:20:32\t[ ] #tag1 0726.4"]);
}

#[test]
fn task_tag2() {
    let output = run_task(&["--task", "--tag", "tag2", "--limit", "10"]);
    let lines = output_lines(&output);
    assert_eq!(lines, vec!["2025-07-27", "\t09:32:16\t[ ] #tag2 0727.1"]);
}

#[test]
fn task_tag1_and_tag2() {
    let output = run_task(&["--task", "--tag", "tag1,tag2", "--limit", "25"]);
    let lines = output_lines(&output);
    assert_eq!(
        lines,
        vec![
            "2025-07-27",
            "\t09:32:16\t[ ] #tag2 0727.1",
            "2025-07-26",
            "\t15:20:32\t[ ] #tag1 0726.4",
        ]
    );
}

#[test]
fn task_tag3_has_no_results() {
    let output = run_task(&["--task", "--tag", "tag3", "--limit", "10"]);
    let lines = output_lines(&output);
    assert!(lines.is_empty());
}

#[test]
fn task_limit() {
    let output = run_task(&["--task", "--limit", "3"]);
    let lines = output_lines(&output);

    assert_eq!(
        lines,
        vec![
            "2025-07-29",
            "\t16:37:54\t[ ] 0729.4",
            "2025-07-28",
            "\t21:19:25\t[ ] 0728.5",
            "2025-07-27",
            "\t17:21:47\t[ ] 0727.4",
        ]
    );
}
