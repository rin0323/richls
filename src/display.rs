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
    let mark = if entry.is_new { "🆕" } else { "" };
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
