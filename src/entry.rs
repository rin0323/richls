use std::ffi::OsStr;
use std::fs::{self, Metadata};
use std::io;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{metadata, pdf, readme};

#[derive(Debug, Clone)]
pub struct EntryInfo {
    pub name: String,
    pub display_name: String,
    pub mode: String,
    pub links: u64,
    pub owner: String,
    pub group: String,
    pub size: u64,
    pub size_human: String,
    pub mtime: SystemTime,
    pub mtime_text: String,
    pub is_new: bool,
    pub info: Option<String>,
}

impl EntryInfo {
    pub fn from_path(path: PathBuf, with_rich_info: bool) -> io::Result<Self> {
        let metadata = fs::symlink_metadata(&path)?;
        let name = path
            .file_name()
            .unwrap_or_else(|| OsStr::new("."))
            .to_string_lossy()
            .into_owned();
        let display_name = if metadata.is_dir() {
            format!("{name}/")
        } else {
            name.clone()
        };
        let mtime = metadata.modified().unwrap_or(UNIX_EPOCH);
        let info = if with_rich_info {
            read_rich_info(&path, &metadata)
        } else {
            None
        };

        Ok(Self {
            mode: metadata::mode_string(&metadata),
            links: metadata::link_count(&metadata),
            owner: metadata::owner_name(&metadata),
            group: metadata::group_name(&metadata),
            size: metadata.len(),
            size_human: metadata::human_size(metadata.len()),
            mtime,
            mtime_text: metadata::format_system_time(mtime),
            is_new: is_new(&metadata),
            name,
            display_name,
            info,
        })
    }
}

fn read_rich_info(path: &std::path::Path, metadata: &Metadata) -> Option<String> {
    if metadata.is_dir() {
        return readme::read_readme_tagline(path);
    }

    if pdf::is_pdf(path) {
        return pdf::read_pdf_title(path);
    }

    None
}

fn is_new(metadata: &Metadata) -> bool {
    let created_or_modified = metadata.created().or_else(|_| metadata.modified()).ok();
    created_or_modified
        .and_then(|time| SystemTime::now().duration_since(time).ok())
        .is_some_and(|age| age <= Duration::from_secs(24 * 60 * 60))
}
