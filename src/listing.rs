use std::fs;
use std::io;

use crate::cli::Config;
use crate::entry::EntryInfo;
use crate::ignore::{self, IgnoreRules};

/// Collects the entries selected by the CLI configuration.
pub fn collect_entries(config: &Config) -> Result<Vec<EntryInfo>, String> {
    collect(config).map_err(|err| format!("cannot access '{}': {err}", config.path.display()))
}

fn collect(config: &Config) -> io::Result<Vec<EntryInfo>> {
    let metadata = fs::symlink_metadata(&config.path)?;
    if metadata.is_file() || metadata.file_type().is_symlink() {
        return EntryInfo::from_path(config.path.clone(), config.long).map(|entry| vec![entry]);
    }
    if !metadata.is_dir() {
        return Ok(Vec::new());
    }

    collect_directory(config)
}

fn collect_directory(config: &Config) -> io::Result<Vec<EntryInfo>> {
    let ignore_rules = load_ignore_rules(config);
    let mut entries = Vec::new();

    for entry in fs::read_dir(&config.path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if should_include(config, &ignore_rules, &name, &entry.path()) {
            entries.push(EntryInfo::from_path(entry.path(), config.long)?);
        }
    }

    Ok(entries)
}

fn load_ignore_rules(config: &Config) -> IgnoreRules {
    if config.respect_ignore {
        IgnoreRules::load(&config.path)
    } else {
        IgnoreRules::empty()
    }
}

fn should_include(
    config: &Config,
    rules: &IgnoreRules,
    name: &str,
    path: &std::path::Path,
) -> bool {
    let hidden = !config.all && ignore::is_hidden(name);
    let ignored = config.respect_ignore && rules.ignores(name, path);
    !hidden && !ignored
}
