use chrono::DateTime;
use std::{
    fs,
    process::{Command, Output},
};

/// Run qwa with the given config path and arguments, returning the output.
pub fn run_qwa(config_path: &str, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_qwa"))
        .args(["--config", config_path])
        .args(args)
        .output()
        .expect("Failed to Run: qwa")
}

#[allow(dead_code)]
pub fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "qwa failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[allow(dead_code)]
pub fn remove_output(path: &str) {
    let _ = fs::remove_file(path);
}

#[allow(dead_code)]
pub fn read_output(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to Read: Expected File")
}

#[allow(dead_code)]
pub fn parse_datetime_from_output(output: &Output, format: &str) -> String {
    DateTime::parse_from_str(
        String::from_utf8_lossy(&output.stdout).trim(),
        "%Y-%m-%d %H:%M:%S%.f %:z",
    )
    .expect("Failed to Parse: Time")
    .format(format)
    .to_string()
}
