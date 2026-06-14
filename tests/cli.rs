use std::{
    fs::{self, File},
    io::Write,
    process::Command,
};
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

#[test]
fn ignore_case_matches_without_changing_output_content() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "NEEDLE IN THE HAYSTACK").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_mini-grep"))
        .arg("--ignore-case")
        .arg("needle")
        .arg(file.path())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("{}:1: NEEDLE IN THE HAYSTACK\n", file.path().display())
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn recursive_search_includes_nested_files() {
    let temp_dir = tempfile::tempdir().unwrap();
    let nested_dir = temp_dir.path().join("z_nested");
    fs::create_dir(&nested_dir).unwrap();

    let root_path = temp_dir.path().join("a_root.txt");
    let nested_path = nested_dir.join("match.txt");

    let mut root_file = File::create(&root_path).unwrap();
    writeln!(root_file, "needle in root").unwrap();

    let mut nested_file = File::create(&nested_path).unwrap();
    writeln!(nested_file, "needle in nested file").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_mini-grep"))
        .arg("--recursive")
        .arg("needle")
        .arg(temp_dir.path())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!(
            "{}:1: needle in root\n{}:1: needle in nested file\n",
            root_path.display(),
            nested_path.display()
        )
    );
    assert!(output.stderr.is_empty());
}
