mod utils;
use utils::{assert_success, read_output, remove_output, run_qwa};

const WORKSPACE_CONFIG_DIR: &str = "./tests/config/workspace";

#[test]
fn test_workspace_multiple_configs_separate_base_directories() {
    let output_a = "./tests/output/workspace_a/aaa.md";
    let output_b = "./tests/output/workspace_b/bbb.md";

    remove_output(output_a);
    remove_output(output_b);

    // Command `aaa` should output to workspace_a
    let output = run_qwa(
        WORKSPACE_CONFIG_DIR,
        &["--add", "aaa", "Message for A"],
    );
    assert_success(&output);
    assert_eq!(read_output(output_a), "- Message for A");

    // Command `bbb` should output to workspace_b
    let output = run_qwa(
        WORKSPACE_CONFIG_DIR,
        &["--add", "bbb", "Message for B"],
    );
    assert_success(&output);
    assert_eq!(read_output(output_b), "- Message for B");

    // Command `--list aaa` should read from workspace_a
    let output_list_a = run_qwa(WORKSPACE_CONFIG_DIR, &["--all", "aaa"]);
    assert_success(&output_list_a);
    assert!(String::from_utf8_lossy(&output_list_a.stdout).contains("Message for A"));

    // Command `--list bbb` should read from workspace_b
    let output_list_b = run_qwa(WORKSPACE_CONFIG_DIR, &["--all", "bbb"]);
    assert_success(&output_list_b);
    assert!(String::from_utf8_lossy(&output_list_b.stdout).contains("Message for B"));
}

