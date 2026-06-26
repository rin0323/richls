use std::fs;
use std::path::Path;

pub fn read_readme_tagline(dir: &Path) -> Option<String> {
    let content = fs::read_to_string(dir.join("README.md")).ok()?;

    let mut in_code_block = false;
    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block || line.is_empty() {
            continue;
        }

        let tagline = line.trim_start_matches('#').trim();
        if !tagline.is_empty() {
            return Some(format!("README: {}", truncate(tagline, 80)));
        }
    }

    None
}

fn truncate(value: &str, max_chars: usize) -> String {
    let mut chars = value.chars();
    let truncated: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{truncated}...")
    } else {
        truncated
    }
}
