use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::tempdir;

fn cargo_run() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rsubst"))
}

#[test]
fn test_simple_output() {
    let dir = tempdir().unwrap();
    let template_path = dir.path().join("template.j2");

    fs::write(&template_path, "Hello {{NAME}}!").unwrap();

    let output = cargo_run()
        .arg(&template_path)
        .env("NAME", "World")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("Failed to convert stdout to string"),
        "Hello World!\n"
    );
}

#[test]
fn test_filters() {
    let dir = tempdir().unwrap();
    let template_path = dir.path().join("template.j2");

    fs::write(
        &template_path,
        "{% for item in ITEMS | split(',') -%}
-{{ item }}
{% endfor %}",
    )
    .unwrap();

    let output = cargo_run()
        .arg(&template_path)
        .env("ITEMS", "a,b,c")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "-a\n-b\n-c\n\n");
}

#[test]
fn test_stdin_input() {
    let mut child = cargo_run()
        .env("NAME", "World")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"Hello {{NAME}}!").unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello World!\n");
}

#[test]
fn test_missing_template_file() {
    let output = cargo_run().arg("nonexistent.j2").output().unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    insta::assert_snapshot!(stderr);
}

#[test]
fn test_missing_variable_silent_without_strict() {
    let dir = tempdir().unwrap();
    let template_path = dir.path().join("template.j2");

    fs::write(&template_path, "Hello {{NAME}}!").unwrap();

    let output = cargo_run().arg(&template_path).output().unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    insta::assert_snapshot!(stdout);
}

#[test]
fn test_strict_errors_on_missing_variable() {
    let dir = tempdir().unwrap();
    let template_path = dir.path().join("template.j2");

    fs::write(&template_path, "Hello {{NAME}}!").unwrap();

    let output = cargo_run()
        .arg("--strict")
        .arg(&template_path)
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    insta::assert_snapshot!(stderr);
}

#[test]
fn test_strict_succeeds_when_variable_present() {
    let dir = tempdir().unwrap();
    let template_path = dir.path().join("template.j2");

    fs::write(&template_path, "Hello {{NAME}}!").unwrap();

    let output = cargo_run()
        .arg("--strict")
        .arg(&template_path)
        .env("NAME", "World")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello World!\n");
}

#[test]
fn test_help_flag() {
    let output = cargo_run().arg("--help").output().unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    insta::assert_snapshot!(stdout);
}
