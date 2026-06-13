use std::{io::Write, process::Command};
use tempfile::NamedTempFile;

#[test]
fn prints_matches_from_file() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "first line").unwrap();
    writeln!(file, "needle in the haystack").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_mini-grep"))
        .arg("needle")
        .arg(file.path())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("{}:2: needle in the haystack\n", file.path().display())
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn exits_with_usage_when_arguments_are_missing() {
    let output = Command::new(env!("CARGO_BIN_EXE_mini-grep"))
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr).unwrap().contains("Usage:"));
    assert!(output.stdout.is_empty());
}
