mod clean_suggest;
mod cli;
mod display;
mod entry;
mod gencomp;
mod ignore;
mod listing;
mod metadata;
mod pdf;
mod sort;

use std::path::Path;

fn main() {
    if let Err(err) = run() {
        eprintln!("richls: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let config = cli::parse_args();

    if config.completions {
        gencomp::generate(Path::new("completions")).map_err(|err| err.to_string())?;
        return Ok(());
    }

    let mut entries = listing::collect_entries(&config)?;
    sort::sort_entries(&mut entries, config.sort_key);

    if config.clean_suggest {
        let suggestions = clean_suggest::collect_clean_suggestions(&entries);
        println!("{}", clean_suggest::format_clean_suggestions(&suggestions));
        return Ok(());
    }

    for line in display::format_entries(&entries, &config) {
        println!("{line}");
    }

    Ok(())
}
