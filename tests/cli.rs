use std::fs;
use std::time::{Duration, SystemTime};

use assert_cmd::Command;
use filetime::{FileTime, set_file_mtime};
use predicates::prelude::*;
use tempfile::tempdir;

fn richls() -> Command {
    Command::cargo_bin("richls").expect("richls binary should build")
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
fn clean_suggest_lists_cleanup_candidates() {
    let directory = tempdir().expect("temporary directory should be created");

    for (name, contents) in [
        ("report_copy.pdf", "copy"),
        ("資料 のコピー.pdf", "copy"),
        ("report (1).pdf", "copy"),
        ("image (2).png", "copy"),
        ("main_old.rs", "backup"),
        ("report_backup.pdf", "backup"),
        ("data.bak", "backup"),
        ("temp.tmp", "temporary"),
        ("memo.swp", "temporary"),
        ("note.txt~", "temporary"),
        ("normal.txt", "normal"),
        ("report_copy_old.pdf", "multiple"),
        ("archive.txt", "old"),
    ] {
        fs::write(directory.path().join(name), contents).expect("fixture should be written");
    }

    fs::write(directory.path().join("empty.txt"), "").expect("empty fixture should be written");
    let old_mtime =
        FileTime::from_system_time(SystemTime::now() - Duration::from_secs(181 * 24 * 60 * 60));
    set_file_mtime(directory.path().join("archive.txt"), old_mtime)
        .expect("archive fixture mtime should be set");
    set_file_mtime(directory.path().join("report_copy_old.pdf"), old_mtime)
        .expect("multiple fixture mtime should be set");
    fs::create_dir(directory.path().join("directory_copy"))
        .expect("directory fixture should exist");

    let output = richls()
        .current_dir(directory.path())
        .arg("--clean-suggest")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).expect("stdout should be utf8");

    assert!(stdout.contains("Clean suggestions:"));
    assert!(stdout.contains("report_copy.pdf"));
    assert!(stdout.contains("資料 のコピー.pdf"));
    assert!(stdout.contains("report (1).pdf"));
    assert!(stdout.contains("image (2).png"));
    assert!(stdout.contains("main_old.rs"));
    assert!(stdout.contains("report_backup.pdf"));
    assert!(stdout.contains("data.bak"));
    assert!(stdout.contains("empty.txt"));
    assert!(stdout.contains("archive.txt"));
    assert!(stdout.contains("temp.tmp"));
    assert!(stdout.contains("memo.swp"));
    assert!(stdout.contains("note.txt~"));
    assert!(stdout.contains("copied file name"));
    assert!(stdout.contains("backup-like file name"));
    assert!(stdout.contains("empty file"));
    assert!(stdout.contains("not modified for 180+ days"));
    assert!(stdout.contains("temporary file"));
    assert!(stdout.contains("report_copy_old.pdf"));
    assert!(stdout.contains("copied file name, backup-like file name, not modified for 180+ days"));
    assert!(!stdout.contains("normal.txt"));
    assert!(!stdout.contains("directory_copy"));
}
