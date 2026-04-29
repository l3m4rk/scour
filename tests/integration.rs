use std::io::Write;
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

#[test]
fn test_stdin_search() {
    use std::io::Write;

    let mut child = Command::new("cargo")
        .args(["run", "--", "fn"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn scour");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"let x = 1;\nfn main() {\n    println!(\"hello\");\n}")
        .unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(stdout.contains("fn main()"));
}
