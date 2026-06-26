use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    Name,
    Size,
    Mtime,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub path: PathBuf,
    pub long: bool,
    pub all: bool,
    pub respect_ignore: bool,
    pub sort_key: SortKey,
}

pub fn parse_args<I>(args: I) -> Result<Config, String>
where
    I: IntoIterator<Item = String>,
{
    let mut config = Config {
        path: PathBuf::from("."),
        long: false,
        all: false,
        respect_ignore: false,
        sort_key: SortKey::Name,
    };
    let mut positional_path = None;
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--long" => config.long = true,
            "--all" => config.all = true,
            "--respect-ignore" => config.respect_ignore = true,
            "--humanize" | "--tagline" | "--pdf-title" | "--new-mark" => {
                config.long = true;
            }
            "--sort" => {
                let Some(key) = args.next() else {
                    return Err("--sort requires one of: name, size, mtime".to_string());
                };
                config.sort_key = parse_sort_key(&key)?;
            }
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-V" | "--version" => {
                println!("richls {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            _ if arg.starts_with("--sort=") => {
                let key = arg.trim_start_matches("--sort=");
                config.sort_key = parse_sort_key(key)?;
            }
            _ if arg.starts_with('-') && arg.len() > 1 => parse_short_options(&arg, &mut config)?,
            _ => {
                if positional_path.is_some() {
                    return Err("only one path can be specified".to_string());
                }
                positional_path = Some(PathBuf::from(arg));
            }
        }
    }

    if let Some(path) = positional_path {
        config.path = path;
    }

    Ok(config)
}

fn parse_short_options(arg: &str, config: &mut Config) -> Result<(), String> {
    for option in arg.trim_start_matches('-').chars() {
        match option {
            'l' => config.long = true,
            'a' => config.all = true,
            'h' => {
                print_help();
                std::process::exit(0);
            }
            'V' => {
                println!("richls {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            _ => return Err(format!("unknown option: -{option}")),
        }
    }
    Ok(())
}

fn parse_sort_key(key: &str) -> Result<SortKey, String> {
    match key {
        "name" => Ok(SortKey::Name),
        "size" => Ok(SortKey::Size),
        "mtime" => Ok(SortKey::Mtime),
        _ => Err(format!(
            "invalid sort key: {key}\navailable values: name, size, mtime"
        )),
    }
}

fn print_help() {
    println!(
        "\
Usage:
  richls [OPTIONS] [FILE]

Options:
  -l, --long              Show ls -l style metadata plus rich info
  -a, --all               Show hidden files
      --respect-ignore    Hide entries matched by .gitignore/.dockerignore
      --sort <key>        Sort by name, size, or mtime
      --humanize          Compatibility alias: enabled by -l
      --tagline           Compatibility alias: enabled by -l
      --pdf-title         Compatibility alias: enabled by -l
      --new-mark          Compatibility alias: enabled by -l
  -h, --help              Show help
  -V, --version           Show version"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_defaults() {
        let config = parse_args(Vec::<String>::new()).unwrap();
        assert_eq!(config.path, PathBuf::from("."));
        assert_eq!(config.sort_key, SortKey::Name);
        assert!(!config.long);
        assert!(!config.all);
    }

    #[test]
    fn parses_combined_short_options() {
        let config = parse_args(["-la".to_string(), "src".to_string()]).unwrap();
        assert!(config.long);
        assert!(config.all);
        assert_eq!(config.path, PathBuf::from("src"));
    }

    #[test]
    fn parses_sort() {
        let config = parse_args(["--sort=size".to_string()]).unwrap();
        assert_eq!(config.sort_key, SortKey::Size);
    }
}
