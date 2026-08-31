mod utils;
use utils::run_qwa;
const CONFIG: &str = "./tests/config/date.toml";

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
fn date_format_list() {
    let output = run_list(&["--list", "line", "--limit", "25"]);
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

// #[test]
// fn date_format_slash() {
//     let output = run_list(&["--list", "slash", "--limit", "25"]);
//     let lines = output_lines(&output);

//     assert_eq!(lines.first(), Some(&"2025-07-29"));
//     assert_eq!(lines.last(), Some(&"\t12:12:12\t0725.1"));
//     assert!(
//         lines
//             .windows(2)
//             .any(|pair| pair == ["2025-07-28", "\t21:19:25\t[ ] 0728.5"])
//     );
//     assert!(lines.contains(&"\t19:00:00\t#tag2 0729.5"));
//     assert!(lines.iter().any(|line| line.contains("0725.5")));
// }

// #[test]
// fn date_format_header() {
//     let output = run_list(&["--list", "header", "--limit", "25"]);
//     let lines = output_lines(&output);

//     assert_eq!(lines.first(), Some(&"2025-07-29"));
//     assert_eq!(lines.last(), Some(&"\t12:12:12\t0725.1"));
//     assert!(
//         lines
//             .windows(2)
//             .any(|pair| pair == ["2025-07-28", "\t21:19:25\t[ ] 0728.5"])
//     );
//     assert!(lines.contains(&"\t19:00:00\t#tag2 0729.5"));
//     assert!(lines.iter().any(|line| line.contains("0725.5")));
// }