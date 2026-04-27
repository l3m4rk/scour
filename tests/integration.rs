use std::process::Command;

fn run_scour(args: &[&str]) -> String {
    let output = Command::new("cargo")
        .args(["run", "--"])
        .args(args)
        .output()
        .expect("Failed to run scour");

    String::from_utf8_lossy(&output.stdout).to_string()
}

const TEST_FILE: &'static str = "tests/fixtures/sample.rs";

#[test]
fn test_basic_search() {
    let output = run_scour(&["fn", TEST_FILE]);
    assert!(output.contains("fn main()"));
}

#[test]
fn test_case_insensitive() {
    let output = run_scour(&["-i", "FN", TEST_FILE]);
    assert!(output.contains("fn main()"));
}

#[test]
fn test_line_numbers() {
    let output = run_scour(&["-n", "fn", TEST_FILE]);
    assert!(output.contains("1: fn main()"));
}

#[test]
fn test_no_match() {
    let output = run_scour(&["zzzzzz", TEST_FILE]);
    assert!(output.is_empty());
}