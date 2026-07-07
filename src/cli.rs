use std::path::PathBuf;

#[cfg(test)]
use std::ffi::OsString;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SortKey {
    Name,
    Size,
    Mtime,
}

#[derive(Debug, Clone, Parser)]
#[command(
    version,
    about = "List files with readable metadata and contextual information"
)]
pub struct Config {
    /// Path to list
    #[arg(default_value = ".", value_name = "FILE")]
    pub path: PathBuf,

    /// Show metadata, human-readable sizes, and rich information
    #[arg(short = 'l', long)]
    pub long: bool,

    /// Show hidden files
    #[arg(short = 'a', long)]
    pub all: bool,

    /// Hide entries matched by .gitignore or .dockerignore
    #[arg(long)]
    pub respect_ignore: bool,

    /// Sort by name, size, or mtime
    #[arg(long = "sort", value_enum, default_value = "name", value_name = "KEY")]
    pub sort_key: SortKey,

    /// Generate shell completion files
    #[arg(long = "complete")]
    pub completions: bool,

    // These options were exposed by an earlier CLI. Keep accepting them as
    // compatibility aliases for the complete long format.
    #[arg(long, hide = true)]
    pub humanize: bool,
    #[arg(long, hide = true)]
    pub tagline: bool,
    #[arg(long, hide = true)]
    pub pdf_title: bool,
    #[arg(long, hide = true)]
    pub new_mark: bool,
}

impl Config {
    pub fn long_enabled(&self) -> bool {
        self.long || self.humanize || self.tagline || self.pdf_title || self.new_mark
    }

    fn normalized(mut self) -> Self {
        self.long = self.long_enabled();
        self
    }
}

pub fn parse_args() -> Config {
    Config::parse().normalized()
}

#[cfg(test)]
fn try_parse_args_from<I, T>(args: I) -> Result<Config, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    Config::try_parse_from(args).map(Config::normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_defaults() {
        let config = try_parse_args_from(["richls"]).unwrap();
        assert_eq!(config.path, PathBuf::from("."));
        assert_eq!(config.sort_key, SortKey::Name);
        assert!(!config.long);
        assert!(!config.all);
    }

    #[test]
    fn parses_combined_short_options() {
        let config = try_parse_args_from(["richls", "-la", "src"]).unwrap();
        assert!(config.long);
        assert!(config.all);
        assert_eq!(config.path, PathBuf::from("src"));
    }

    #[test]
    fn parses_sort() {
        let config = try_parse_args_from(["richls", "--sort=size"]).unwrap();
        assert_eq!(config.sort_key, SortKey::Size);
    }

    #[test]
    fn compatibility_options_enable_long_format() {
        for option in ["--humanize", "--tagline", "--pdf-title", "--new-mark"] {
            let config = try_parse_args_from(["richls", option]).unwrap();
            assert!(config.long, "{option} should enable long format");
        }
    }
}
