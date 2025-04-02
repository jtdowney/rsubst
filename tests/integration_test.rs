use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::tempdir;

#[test]
fn test_simple_output() {
    let dir = tempdir().unwrap();
    let template_path = dir.path().join("template.j2");

    fs::write(&template_path, "Hello {{NAME}}!").unwrap();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(template_path)
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

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(template_path)
        .env("ITEMS", "a,b,c")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "-a\n-b\n-c\n\n");
}

#[test]
fn test_stdin_input() {
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--")
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
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("nonexistent.j2")
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Failed to read template file"));
}
