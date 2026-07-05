mod cli;
mod display;
mod entry;
mod gencomp;
mod ignore;
mod metadata;
mod pdf;
mod readme;
mod sort;

use std::fs;
use std::io;
use std::path::Path;

use cli::Config;
use entry::EntryInfo;

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

    let mut entries = collect_entries(&config).map_err(|err| format_io_error(&config, err))?;

    sort::sort_entries(&mut entries, config.sort_key);
    display::print_entries(&entries, &config);

    Ok(())
}

fn collect_entries(config: &Config) -> io::Result<Vec<EntryInfo>> {
    let metadata = fs::symlink_metadata(&config.path)?;

    if metadata.is_file() || metadata.file_type().is_symlink() {
        return EntryInfo::from_path(config.path.clone(), config.long).map(|entry| vec![entry]);
    }

    if !metadata.is_dir() {
        return Ok(Vec::new());
    }

    let ignore_rules = if config.respect_ignore {
        ignore::IgnoreRules::load(&config.path)
    } else {
        ignore::IgnoreRules::empty()
    };

    let mut entries = Vec::new();
    for entry in fs::read_dir(&config.path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();

        if !config.all && ignore::is_hidden(&name) {
            continue;
        }
        if config.respect_ignore && ignore_rules.ignores(&name, &entry.path()) {
            continue;
        }

        entries.push(EntryInfo::from_path(entry.path(), config.long)?);
    }

    Ok(entries)
}

fn format_io_error(config: &Config, err: io::Error) -> String {
    format!("cannot access '{}': {err}", config.path.display())
}
