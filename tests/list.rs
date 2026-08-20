mod utils;
use utils::run_qwa;

const CONFIG: &str = "./tests/config/list.toml";

fn run_list(args: &[&str]) -> String {
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
fn normal_list() {
    let output = run_list(&["--list", "--limit", "25"]);
    let lines = output_lines(&output);

    assert_eq!(lines.first(), Some(&"2025-07-29"));
    assert_eq!(lines.last(), Some(&"\t12:12:12\t0725.1"));
    assert!(
        lines
            .windows(2)
            .any(|pair| pair == ["2025-07-28", "\t21:19:25\t[ ] 0728.5"])
    );
    assert!(lines.contains(&"\t19:00:00\t#tag2 0729.5"));
    assert!(lines.iter().any(|line| line.contains("0725.5")));
}

#[test]
fn list_tag1() {
    let output = run_list(&["--list", "--tag", "tag1", "--limit", "10"]);
    let lines = output_lines(&output);
    assert_eq!(
        lines,
        vec![
            "2025-07-29",
            "\t11:48:36\t#tag1 0729.3",
            "\t11:47:45\t#tag1 #tag2 0729.2",
            "2025-07-27",
            "\t17:21:38\t#tag1 0727.3",
            "2025-07-26",
            "\t21:32:12\t#tag1 #tag2 0726.5",
            "\t15:20:32\t[ ] #tag1 0726.4",
            "2025-07-25",
            "\t16:10:12\t#tag1 0725.5",
        ]
    );
}

#[test]
fn list_tag2() {
    let output = run_list(&["--list", "--tag", "tag2", "--limit", "10"]);
    let lines = output_lines(&output);
    assert_eq!(
        lines,
        vec![
            "2025-07-29",
            "\t19:00:00\t#tag2 0729.5",
            "\t11:47:45\t#tag1 #tag2 0729.2",
            "2025-07-28",
            "\t10:23:23\t#tag2 0728.3",
            "2025-07-27",
            "\t09:32:16\t[ ] #tag2 0727.1",
            "2025-07-26",
            "\t21:32:12\t#tag1 #tag2 0726.5",
            "\t17:24:35\t#tag2 0726.2",
        ]
    );
}

#[test]
fn list_tag1_and_tag2() {
    let output = run_list(&["--list", "--tag", "tag1,tag2", "--limit", "10"]);
    let lines = output_lines(&output);
    assert_eq!(
        lines,
        vec![
            "2025-07-29",
            "\t19:00:00\t#tag2 0729.5",
            "\t11:48:36\t#tag1 0729.3",
            "\t11:47:45\t#tag1 #tag2 0729.2",
            "2025-07-28",
            "\t10:23:23\t#tag2 0728.3",
            "2025-07-27",
            "\t17:21:38\t#tag1 0727.3",
            "\t09:32:16\t[ ] #tag2 0727.1",
            "2025-07-26",
            "\t21:32:12\t#tag1 #tag2 0726.5",
            "\t17:24:35\t#tag2 0726.2",
            "\t15:20:32\t[ ] #tag1 0726.4",
            "2025-07-25",
            "\t16:10:12\t#tag1 0725.5",
        ]
    );
}

#[test]
fn list_tag3_has_no_results() {
    let output = run_list(&["--list", "--tag", "tag3", "--limit", "10"]);
    let lines = output_lines(&output);
    assert!(lines.is_empty());
}

#[test]
fn list_limit() {
    let output = run_list(&["--list", "--limit", "3"]);
    let lines = output_lines(&output);

    assert_eq!(
        lines,
        vec![
            "2025-07-29",
            "\t19:00:00\t#tag2 0729.5",
            "\t16:37:54\t[ ] 0729.4",
            "\t11:48:36\t#tag1 0729.3",
        ]
    );
}
