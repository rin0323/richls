use std::time::{Duration, SystemTime};

use crate::entry::EntryInfo;

const OLD_FILE_DAYS: u64 = 180;
const SECONDS_PER_DAY: u64 = 24 * 60 * 60;

const COPIED_REASON: &str = "copied file name";
const BACKUP_REASON: &str = "backup-like file name";
const EMPTY_REASON: &str = "empty file";
const OLD_REASON: &str = "not modified for 180+ days";
const TEMPORARY_REASON: &str = "temporary file";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanSuggestion {
    pub name: String,
    pub size_human: String,
    pub mtime_text: String,
    pub reasons: Vec<String>,
}

pub fn collect_clean_suggestions(entries: &[EntryInfo]) -> Vec<CleanSuggestion> {
    collect_clean_suggestions_with_now(entries, SystemTime::now())
}

pub fn format_clean_suggestions(suggestions: &[CleanSuggestion]) -> String {
    let mut lines = vec!["Clean suggestions:".to_string()];
    lines.extend(suggestions.iter().map(format_clean_suggestion));
    lines.join("\n")
}

fn collect_clean_suggestions_with_now(
    entries: &[EntryInfo],
    now: SystemTime,
) -> Vec<CleanSuggestion> {
    entries
        .iter()
        .filter_map(|entry| clean_suggestion(entry, now))
        .collect()
}

fn clean_suggestion(entry: &EntryInfo, now: SystemTime) -> Option<CleanSuggestion> {
    let reasons = clean_suggestion_reasons(entry, now);
    (!reasons.is_empty()).then(|| CleanSuggestion {
        name: entry.name.clone(),
        size_human: entry.size_human.clone(),
        mtime_text: entry.mtime_text.clone(),
        reasons,
    })
}

fn clean_suggestion_reasons(entry: &EntryInfo, now: SystemTime) -> Vec<String> {
    if !entry.is_regular_file {
        return Vec::new();
    }

    clean_suggestion_reasons_for_file(entry, now)
}

fn clean_suggestion_reasons_for_file(entry: &EntryInfo, now: SystemTime) -> Vec<String> {
    let mut reasons = Vec::new();
    push_reason(&mut reasons, is_copy_like_name(&entry.name), COPIED_REASON);
    push_reason(
        &mut reasons,
        is_backup_like_name(&entry.name),
        BACKUP_REASON,
    );
    push_reason(&mut reasons, entry.size == 0, EMPTY_REASON);
    push_reason(&mut reasons, is_old_file(entry.mtime, now), OLD_REASON);
    push_reason(
        &mut reasons,
        is_temporary_file_name(&entry.name),
        TEMPORARY_REASON,
    );
    reasons
}

fn push_reason(reasons: &mut Vec<String>, detected: bool, reason: &str) {
    if detected {
        reasons.push(reason.to_string());
    }
}

fn is_copy_like_name(name: &str) -> bool {
    let name = name.to_lowercase();
    name.contains("copy") || name.contains("コピー") || has_windows_copy_number(&name)
}

fn has_windows_copy_number(name: &str) -> bool {
    let bytes = name.as_bytes();

    for start in 0..bytes.len() {
        if bytes[start] == b'(' && has_closing_copy_number(bytes, start) {
            return true;
        }
    }

    false
}

fn has_closing_copy_number(bytes: &[u8], start: usize) -> bool {
    let mut end = start + 1;
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }

    end > start + 1 && end < bytes.len() && bytes[end] == b')'
}

fn is_backup_like_name(name: &str) -> bool {
    let name = name.to_lowercase();
    name.contains("old") || name.contains("backup") || name.contains("bak")
}

fn is_temporary_file_name(name: &str) -> bool {
    let name = name.to_lowercase();
    name.ends_with(".tmp") || name.ends_with(".swp") || name.ends_with('~')
}

fn is_old_file(mtime: SystemTime, now: SystemTime) -> bool {
    let threshold = Duration::from_secs(OLD_FILE_DAYS * SECONDS_PER_DAY);
    now.duration_since(mtime).is_ok_and(|age| age >= threshold)
}

fn format_clean_suggestion(suggestion: &CleanSuggestion) -> String {
    format!(
        "  {:<24} {:>8} {:<16} {}",
        suggestion.name,
        suggestion.size_human,
        suggestion.mtime_text,
        suggestion.reasons.join(", ")
    )
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, UNIX_EPOCH};

    use super::*;

    fn entry(name: &str, size: u64, mtime: SystemTime, is_regular_file: bool) -> EntryInfo {
        EntryInfo {
            name: name.to_string(),
            display_name: name.to_string(),
            mode: "-rw-r--r--".to_string(),
            links: 1,
            owner: "rin".to_string(),
            group: "staff".to_string(),
            size,
            size_human: format!("{size}B"),
            mtime,
            mtime_text: "2026-07-01 00:00".to_string(),
            is_new: false,
            is_regular_file,
            info: None,
        }
    }

    fn now() -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(365 * SECONDS_PER_DAY)
    }

    fn recent_time() -> SystemTime {
        now() - Duration::from_secs(30 * SECONDS_PER_DAY)
    }

    fn old_time() -> SystemTime {
        now() - Duration::from_secs(181 * SECONDS_PER_DAY)
    }

    fn reasons_for_name(name: &str) -> Vec<String> {
        let entry = entry(name, 1, recent_time(), true);
        clean_suggestion_reasons(&entry, now())
    }

    #[test]
    fn detects_english_copy_names() {
        assert_eq!(
            reasons_for_name("report_copy.pdf"),
            vec![COPIED_REASON.to_string()]
        );
    }

    #[test]
    fn detects_japanese_copy_names() {
        assert_eq!(
            reasons_for_name("資料_コピー.pdf"),
            vec![COPIED_REASON.to_string()]
        );
    }

    #[test]
    fn detects_macos_copy_names() {
        assert_eq!(
            reasons_for_name("資料 のコピー.pdf"),
            vec![COPIED_REASON.to_string()]
        );
    }

    #[test]
    fn detects_windows_copy_names() {
        for name in ["report (1).pdf", "image (2).png", "data(10).csv"] {
            assert_eq!(reasons_for_name(name), vec![COPIED_REASON.to_string()]);
        }
    }

    #[test]
    fn detects_backup_like_names() {
        for name in ["main_old.rs", "report_backup.pdf", "data.bak"] {
            assert_eq!(reasons_for_name(name), vec![BACKUP_REASON.to_string()]);
        }
    }

    #[test]
    fn detects_temporary_file_names() {
        for name in ["temp.tmp", "memo.swp", "note.txt~"] {
            assert_eq!(reasons_for_name(name), vec![TEMPORARY_REASON.to_string()]);
        }
    }

    #[test]
    fn ignores_normal_file_names() {
        assert!(reasons_for_name("report.txt").is_empty());
    }

    #[test]
    fn detects_names_case_insensitively() {
        assert_eq!(
            reasons_for_name("COPY_OLD.SWP"),
            vec![
                COPIED_REASON.to_string(),
                BACKUP_REASON.to_string(),
                TEMPORARY_REASON.to_string(),
            ]
        );
    }

    #[test]
    fn ignores_out_of_scope_version_words() {
        for name in ["final2.txt", "latest.txt", "修正.txt", "最終.txt"] {
            assert!(reasons_for_name(name).is_empty());
        }
    }

    #[test]
    fn returns_multiple_reasons_for_multiple_matches() {
        let entry = entry("report_copy_old.pdf", 1, old_time(), true);

        assert_eq!(
            clean_suggestion_reasons(&entry, now()),
            vec![
                COPIED_REASON.to_string(),
                BACKUP_REASON.to_string(),
                OLD_REASON.to_string(),
            ]
        );
    }

    #[test]
    fn detects_empty_files() {
        let entry = entry("empty.txt", 0, recent_time(), true);
        assert_eq!(
            clean_suggestion_reasons(&entry, now()),
            vec![EMPTY_REASON.to_string()]
        );
    }

    #[test]
    fn detects_old_files() {
        let entry = entry("archive.txt", 1, old_time(), true);
        assert_eq!(
            clean_suggestion_reasons(&entry, now()),
            vec![OLD_REASON.to_string()]
        );
    }

    #[test]
    fn ignores_non_regular_files() {
        let entries = vec![entry("report_copy.pdf", 1, recent_time(), false)];

        assert!(collect_clean_suggestions_with_now(&entries, now()).is_empty());
    }
}
