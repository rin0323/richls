use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct IgnoreRules {
    patterns: Vec<IgnorePattern>,
}

#[derive(Debug, Clone)]
struct IgnorePattern {
    value: String,
    directory_only: bool,
}

impl IgnoreRules {
    pub fn empty() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn load(dir: &Path) -> Self {
        let mut rules = Self::empty();
        for file_name in [".gitignore", ".dockerignore"] {
            rules.load_file(&dir.join(file_name));
        }
        rules
    }

    pub fn ignores(&self, name: &str, path: &Path) -> bool {
        self.patterns
            .iter()
            .any(|pattern| pattern.matches(name, path.is_dir()))
    }

    fn load_file(&mut self, path: &Path) {
        let Ok(contents) = fs::read_to_string(path) else {
            return;
        };

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
                continue;
            }

            let directory_only = line.ends_with('/');
            let Some(value) = normalize_pattern(line) else {
                continue;
            };

            if !value.is_empty() {
                self.patterns.push(IgnorePattern {
                    value,
                    directory_only,
                });
            }
        }
    }
}

fn normalize_pattern(line: &str) -> Option<String> {
    let pattern = line.trim_start_matches('/').trim_end_matches('/');

    if let Some(suffix) = pattern.strip_prefix("**/") {
        return Some(suffix.to_string());
    }

    if pattern.contains('/') {
        let first_segment = pattern.split('/').next()?;
        if first_segment == "*" || first_segment == "**" {
            return None;
        }
        return Some(first_segment.to_string());
    }

    Some(pattern.to_string())
}

impl IgnorePattern {
    fn matches(&self, name: &str, is_dir: bool) -> bool {
        if self.directory_only && !is_dir {
            return false;
        }

        wildcard_match(&self.value, name)
    }
}

pub fn is_hidden(name: &str) -> bool {
    name.starts_with('.')
}

fn wildcard_match(pattern: &str, value: &str) -> bool {
    if pattern == value {
        return true;
    }

    if !pattern.contains('*') {
        return false;
    }

    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.len() == 2 {
        return value.starts_with(parts[0]) && value.ends_with(parts[1]);
    }

    let mut rest = value;
    for part in parts.into_iter().filter(|part| !part.is_empty()) {
        let index = rest.find(part);
        let Some(index) = index else {
            return false;
        };
        rest = &rest[index + part.len()..];
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_simple_wildcards() {
        assert!(wildcard_match("*.log", "debug.log"));
        assert!(wildcard_match("target", "target"));
        assert!(!wildcard_match("*.log", "README.md"));
    }

    #[test]
    fn normalizes_slash_patterns_for_top_level_listing() {
        assert_eq!(normalize_pattern(".vscode/*"), Some(".vscode".to_string()));
        assert_eq!(normalize_pattern("**/*.rs.bk"), Some("*.rs.bk".to_string()));
    }
}
