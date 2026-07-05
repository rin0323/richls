mod cli;
mod gencomp;

use std::cmp::Ordering;
use std::ffi::OsStr;
use std::fs::{self, DirEntry, Metadata};
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use cli::{Config, SortKey};

#[derive(Debug, Clone)]
struct ListingEntry {
    name: String,
    path: PathBuf,
    metadata: Metadata,
}

#[derive(Debug, Clone)]
struct IgnoreRules {
    patterns: Vec<String>,
}

impl IgnoreRules {
    fn empty() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    fn load(dir: &Path) -> Self {
        let mut rules = Self::empty();
        for file_name in [".gitignore", ".dockerignore"] {
            let path = dir.join(file_name);
            let Ok(contents) = fs::read_to_string(path) else {
                continue;
            };
            for line in contents.lines() {
                let pattern = line.trim();
                if pattern.is_empty() || pattern.starts_with('#') || pattern.starts_with('!') {
                    continue;
                }
                rules
                    .patterns
                    .push(pattern.trim_end_matches('/').to_string());
            }
        }
        rules
    }

    fn ignores(&self, name: &str) -> bool {
        self.patterns.iter().any(|pattern| {
            pattern == name
                || pattern.strip_prefix('/').is_some_and(|p| p == name)
                || pattern
                    .strip_suffix('*')
                    .is_some_and(|prefix| name.starts_with(prefix))
        })
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("richls: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let config = cli::parse_args();

    if config.completions {
        gencomp::generate(Path::new("completions"));
        return Ok(());
    }

    if config.tagline {
        print_tagline(&config.path).map_err(|err| err.to_string())?;
    }

    let mut entries = collect_entries(&config).map_err(|err| err.to_string())?;
    sort_entries(&mut entries, config.sort_key);

    for entry in entries {
        println!("{}", format_entry(&entry, &config));
    }

    Ok(())
}

fn collect_entries(config: &Config) -> io::Result<Vec<ListingEntry>> {
    let metadata = fs::metadata(&config.path)?;
    if metadata.is_file() {
        return Ok(vec![ListingEntry {
            name: file_name(&config.path),
            path: config.path.clone(),
            metadata,
        }]);
    }

    let ignore_rules = if config.respect_ignore {
        IgnoreRules::load(&config.path)
    } else {
        IgnoreRules::empty()
    };

    let mut entries = Vec::new();
    for entry in fs::read_dir(&config.path)? {
        let entry = entry?;
        let name = entry_name(&entry);
        if (!config.all && is_hidden(&name)) || ignore_rules.ignores(&name) {
            continue;
        }
        entries.push(ListingEntry {
            name,
            path: entry.path(),
            metadata: entry.metadata()?,
        });
    }

    Ok(entries)
}

fn sort_entries(entries: &mut [ListingEntry], sort_key: SortKey) {
    entries.sort_by(|left, right| match sort_key {
        SortKey::Name => cmp_name(left, right),
        SortKey::Size => left
            .metadata
            .len()
            .cmp(&right.metadata.len())
            .then_with(|| cmp_name(left, right)),
        SortKey::Mtime => modified_time(left)
            .cmp(&modified_time(right))
            .then_with(|| cmp_name(left, right)),
    });
}

fn cmp_name(left: &ListingEntry, right: &ListingEntry) -> Ordering {
    left.name.to_lowercase().cmp(&right.name.to_lowercase())
}

fn format_entry(entry: &ListingEntry, config: &Config) -> String {
    let mut output = if config.long {
        format!(
            "{} {:>8} {:>10} {}",
            file_kind(&entry.metadata),
            format_size(entry.metadata.len(), config.humanize),
            format_mtime(&entry.metadata),
            display_name(entry)
        )
    } else {
        display_name(entry)
    };

    if config.pdf_title && is_pdf(&entry.path) {
        output.push_str(" [pdf]");
    }

    if config.new_mark && is_new(&entry.metadata) {
        output.push_str(" new");
    }

    output
}

fn display_name(entry: &ListingEntry) -> String {
    if entry.metadata.is_dir() {
        format!("{}/", entry.name)
    } else {
        entry.name.clone()
    }
}

fn file_kind(metadata: &Metadata) -> char {
    if metadata.is_dir() {
        'd'
    } else if metadata.is_file() {
        '-'
    } else {
        '?'
    }
}

fn format_size(size: u64, humanize: bool) -> String {
    if !humanize {
        return size.to_string();
    }

    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut value = size as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < units.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{size}B")
    } else {
        format!("{value:.1}{}", units[unit])
    }
}

fn format_mtime(metadata: &Metadata) -> String {
    metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn modified_time(entry: &ListingEntry) -> SystemTime {
    entry.metadata.modified().unwrap_or(UNIX_EPOCH)
}

fn is_new(metadata: &Metadata) -> bool {
    metadata
        .modified()
        .ok()
        .and_then(|time| SystemTime::now().duration_since(time).ok())
        .is_some_and(|age| age <= Duration::from_secs(24 * 60 * 60))
}

fn print_tagline(path: &Path) -> io::Result<()> {
    let readme = if path.is_dir() {
        path.join("README.md")
    } else {
        path.parent()
            .unwrap_or_else(|| Path::new("."))
            .join("README.md")
    };

    let Ok(contents) = fs::read_to_string(readme) else {
        return Ok(());
    };

    if let Some(line) = contents.lines().find(|line| !line.trim().is_empty()) {
        println!("tagline: {}", line.trim());
    }

    Ok(())
}

fn entry_name(entry: &DirEntry) -> String {
    entry.file_name().to_string_lossy().into_owned()
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or_else(|| OsStr::new("."))
        .to_string_lossy()
        .into_owned()
}

fn is_hidden(name: &str) -> bool {
    name.starts_with('.')
}

fn is_pdf(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("pdf"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn humanizes_sizes() {
        assert_eq!(format_size(999, true), "999B");
        assert_eq!(format_size(1536, true), "1.5KB");
    }
}
