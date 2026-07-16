use std::fs;
use std::time::{Duration, SystemTime};

use assert_cmd::Command;
use filetime::{FileTime, set_file_mtime};
use predicates::prelude::*;
use tempfile::tempdir;

fn richls() -> Command {
    Command::cargo_bin("richls").expect("richls binary should build")
}

fn clean_suggest_output(directory: &tempfile::TempDir) -> String {
    let output = richls()
        .current_dir(directory.path())
        .arg("--clean-suggest")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    String::from_utf8(output).expect("stdout should be utf8")
}

fn output_line<'a>(stdout: &'a str, name: &str) -> &'a str {
    stdout
        .lines()
        .find(|line| line.contains(name))
        .unwrap_or_else(|| panic!("output should contain {name}"))
}

#[test]
fn help_documents_main_options_and_long_size_behavior() {
    richls().arg("--help").assert().success().stdout(
        predicate::str::contains("-a, --all")
            .and(predicate::str::contains("-l, --long"))
            .and(predicate::str::contains("--clean-suggest"))
            .and(predicate::str::contains("human-readable")),
    );
}

#[test]
fn hidden_files_require_all_option() {
    let directory = tempdir().expect("temporary directory should be created");
    fs::write(directory.path().join("visible.txt"), "visible")
        .expect("visible fixture should be written");
    fs::write(directory.path().join(".hidden"), "hidden")
        .expect("hidden fixture should be written");

    richls()
        .current_dir(directory.path())
        .assert()
        .success()
        .stdout(
            predicate::str::contains("visible.txt").and(predicate::str::contains(".hidden").not()),
        );

    richls()
        .current_dir(directory.path())
        .arg("-a")
        .assert()
        .success()
        .stdout(predicate::str::contains("visible.txt").and(predicate::str::contains(".hidden")));
}

#[cfg(unix)]
#[test]
fn long_listing_contains_metadata_human_size_and_readable_time() {
    use std::os::unix::fs::PermissionsExt;

    let directory = tempdir().expect("temporary directory should be created");
    let file = directory.path().join("sample.bin");
    fs::write(&file, vec![0_u8; 2048]).expect("binary fixture should be written");
    fs::set_permissions(&file, fs::Permissions::from_mode(0o644))
        .expect("fixture permissions should be set");
    let mtime = FileTime::from_unix_time(1_719_792_000, 0);
    set_file_mtime(&file, mtime).expect("fixture mtime should be set");

    richls()
        .current_dir(directory.path())
        .args(["-l", "sample.bin"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("-rw-r--r--")
                .and(predicate::str::contains(" 1 "))
                .and(predicate::str::contains("2.0KB"))
                .and(predicate::str::contains("2024-"))
                .and(predicate::str::contains("sample.bin"))
                .and(predicate::str::contains("1719792000").not()),
        );
}

#[test]
fn long_listing_displays_directory_readme_tagline() {
    let directory = tempdir().expect("temporary directory should be created");
    let docs = directory.path().join("docs");
    fs::create_dir(&docs).expect("docs fixture should be created");
    fs::write(docs.join("README.md"), "# Fixture documentation\n")
        .expect("README fixture should be written");

    richls()
        .current_dir(directory.path())
        .arg("-l")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("docs/")
                .and(predicate::str::contains("README: Fixture documentation")),
        );
}

#[test]
fn pdf_title_is_displayed_when_available() {
    let directory = tempdir().expect("temporary directory should be created");
    fs::write(
        directory.path().join("titled.pdf"),
        b"%PDF-1.4\n<< /Title (Fixture PDF Title) >>",
    )
    .expect("PDF fixture should be written");

    richls()
        .current_dir(directory.path())
        .args(["-l", "titled.pdf"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("PDF: Fixture PDF Title")
                .and(predicate::str::contains("[pdf]").not()),
        );
}

#[test]
fn missing_pdf_title_leaves_info_column_empty() {
    let directory = tempdir().expect("temporary directory should be created");
    fs::write(
        directory.path().join("untitled.pdf"),
        b"%PDF-1.4\n<< /Author (rin) >>",
    )
    .expect("PDF fixture should be written");

    richls()
        .current_dir(directory.path())
        .args(["-l", "untitled.pdf"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("untitled.pdf")
                .and(predicate::str::contains("PDF:").not())
                .and(predicate::str::contains("[pdf]").not()),
        );
}

#[test]
fn clean_suggest_lists_copy_like_names_without_empty_reason() {
    let directory = tempdir().expect("temporary directory should be created");

    for name in [
        "report_copy.pdf",
        "資料_コピー.pdf",
        "資料のコピー.pdf",
        "資料 のコピー.pdf",
        "report (1).pdf",
        "image (2).png",
        "data(10).csv",
    ] {
        fs::write(directory.path().join(name), "data")
            .expect("copy-like fixture should be written");
    }

    fs::write(directory.path().join("normal.txt"), "data")
        .expect("normal fixture should be written");
    fs::create_dir(directory.path().join("directory_copy"))
        .expect("directory fixture should exist");

    let stdout = clean_suggest_output(&directory);

    assert!(stdout.contains("Clean suggestions:"));
    assert!(!stdout.contains("normal.txt"));
    assert!(!stdout.contains("directory_copy"));

    for name in [
        "report_copy.pdf",
        "資料_コピー.pdf",
        "資料のコピー.pdf",
        "資料 のコピー.pdf",
        "report (1).pdf",
        "image (2).png",
        "data(10).csv",
    ] {
        let line = output_line(&stdout, name);
        assert!(line.contains("copied file name"));
        assert!(!line.contains("empty file"));
    }
}

#[test]
fn clean_suggest_lists_macos_decomposed_copy_name_for_path_argument() {
    let directory = tempdir().expect("temporary directory should be created");
    let macos_decomposed_copy = "資料 のコヒ\u{309a}ー.pdf";
    fs::write(directory.path().join(macos_decomposed_copy), "data")
        .expect("macOS copy-like fixture should be written");

    let output = richls()
        .arg("--clean-suggest")
        .arg(directory.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).expect("stdout should be utf8");
    let line = output_line(&stdout, macos_decomposed_copy);

    assert!(line.contains("copied file name"));
    assert!(!line.contains("empty file"));
}

#[test]
fn clean_suggest_lists_backup_and_temporary_names_without_empty_reason() {
    let directory = tempdir().expect("temporary directory should be created");

    for name in ["main_old.rs", "report_backup.pdf", "data.bak"] {
        fs::write(directory.path().join(name), "data")
            .expect("backup-like fixture should be written");
    }

    for name in ["temp.tmp", "memo.swp", "note.txt~"] {
        fs::write(directory.path().join(name), "data")
            .expect("temporary fixture should be written");
    }

    let stdout = clean_suggest_output(&directory);

    for name in ["main_old.rs", "report_backup.pdf", "data.bak"] {
        let line = output_line(&stdout, name);
        assert!(line.contains("backup-like file name"));
        assert!(!line.contains("empty file"));
    }

    for name in ["temp.tmp", "memo.swp", "note.txt~"] {
        let line = output_line(&stdout, name);
        assert!(line.contains("temporary file"));
        assert!(!line.contains("empty file"));
    }
}

#[test]
fn clean_suggest_lists_empty_files_separately() {
    let directory = tempdir().expect("temporary directory should be created");
    fs::write(directory.path().join("empty.txt"), "").expect("empty fixture should be written");
    fs::write(directory.path().join("normal.txt"), "data")
        .expect("normal fixture should be written");

    let stdout = clean_suggest_output(&directory);
    let line = output_line(&stdout, "empty.txt");

    assert!(line.contains("empty file"));
    assert!(!line.contains("copied file name"));
    assert!(!line.contains("backup-like file name"));
    assert!(!line.contains("temporary file"));
    assert!(!stdout.contains("normal.txt"));
}

#[test]
fn clean_suggest_lists_old_files() {
    let directory = tempdir().expect("temporary directory should be created");
    fs::write(directory.path().join("archive.txt"), "data").expect("old fixture should be written");
    let old_mtime =
        FileTime::from_system_time(SystemTime::now() - Duration::from_secs(181 * 24 * 60 * 60));
    set_file_mtime(directory.path().join("archive.txt"), old_mtime)
        .expect("archive fixture mtime should be set");

    let stdout = clean_suggest_output(&directory);
    let line = output_line(&stdout, "archive.txt");

    assert!(line.contains("not modified for 180+ days"));
    assert!(!line.contains("empty file"));
}

#[test]
fn clean_suggest_lists_multiple_reasons_without_empty_reason() {
    let directory = tempdir().expect("temporary directory should be created");
    fs::write(directory.path().join("report_copy_old.pdf"), "data")
        .expect("multiple fixture should be written");
    let old_mtime =
        FileTime::from_system_time(SystemTime::now() - Duration::from_secs(181 * 24 * 60 * 60));
    set_file_mtime(directory.path().join("report_copy_old.pdf"), old_mtime)
        .expect("multiple fixture mtime should be set");

    let stdout = clean_suggest_output(&directory);
    let line = output_line(&stdout, "report_copy_old.pdf");

    assert!(line.contains("copied file name, backup-like file name"));
    assert!(line.contains("not modified for 180+ days"));
    assert!(!line.contains("empty file"));
}
