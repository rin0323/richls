use crate::{cli::Config, entry::EntryInfo};

pub fn print_entries(entries: &[EntryInfo], config: &Config) {
    for entry in entries {
        if config.long {
            println!("{}", format_long_entry(entry));
        } else {
            println!("{}", entry.display_name);
        }
    }
}

fn format_long_entry(entry: &EntryInfo) -> String {
    let mark = if entry.is_new { "new" } else { "" };
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
}
