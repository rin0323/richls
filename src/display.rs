use crate::{cli::Config, entry::EntryInfo};

/// Converts collected entries into complete output lines.
pub fn format_entries(entries: &[EntryInfo], config: &Config) -> Vec<String> {
    entries
        .iter()
        .map(|entry| format_entry(entry, config.long))
        .collect()
}

fn format_entry(entry: &EntryInfo, long: bool) -> String {
    if long {
        format_long_entry(entry)
    } else {
        entry.display_name.clone()
    }
}

fn format_long_entry(entry: &EntryInfo) -> String {
    let mark = new_mark(entry.is_new);
    let info = entry.info.as_deref().unwrap_or("");

    format!(
        "{:<4} {:<10} {:>2} {:<8} {:<8} {:>8} {:<16} {:<24} {}",
        mark,
        entry.mode,
        entry.links,
        entry.owner,
        entry.group,
        entry.size_human,
        entry.mtime_text,
        entry.display_name,
        info
    )
}

fn new_mark(is_new: bool) -> &'static str {
    if is_new { "new" } else { "" }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use super::*;

    fn entry(is_new: bool) -> EntryInfo {
        EntryInfo {
            name: "report.pdf".to_string(),
            display_name: "report.pdf".to_string(),
            mode: "-rw-r--r--".to_string(),
            links: 1,
            owner: "rin".to_string(),
            group: "staff".to_string(),
            size: 1536,
            size_human: "1.5KB".to_string(),
            mtime: UNIX_EPOCH,
            mtime_text: "2026-07-06 12:00".to_string(),
            is_new,
            info: None,
        }
    }

    #[test]
    fn long_format_uses_human_readable_fields_and_new_marker() {
        let output = format_long_entry(&entry(true));

        assert!(output.starts_with("new "));
        assert!(output.contains("-rw-r--r--"));
        assert!(output.contains("1.5KB"));
        assert!(output.contains("2026-07-06 12:00"));
        assert!(!output.contains("🆕"));
    }

    #[test]
    fn new_marker_matches_documented_text() {
        assert_eq!(new_mark(true), "new");
        assert_eq!(new_mark(false), "");
    }

    #[test]
    fn formats_short_and_long_lines() {
        let entry = entry(false);

        assert_eq!(format_entry(&entry, false), "report.pdf");
        assert!(format_entry(&entry, true).contains("-rw-r--r--"));
    }
}
