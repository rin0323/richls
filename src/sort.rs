use std::cmp::Ordering;

use crate::{cli::SortKey, entry::EntryInfo};

pub fn sort_entries(entries: &mut [EntryInfo], sort_key: SortKey) {
    entries.sort_by(|left, right| match sort_key {
        SortKey::Name => cmp_name(left, right),
        SortKey::Size => right
            .size
            .cmp(&left.size)
            .then_with(|| cmp_name(left, right)),
        SortKey::Mtime => right
            .mtime
            .cmp(&left.mtime)
            .then_with(|| cmp_name(left, right)),
    });
}

fn cmp_name(left: &EntryInfo, right: &EntryInfo) -> Ordering {
    left.name.to_lowercase().cmp(&right.name.to_lowercase())
}
