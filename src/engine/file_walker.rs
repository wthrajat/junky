use std::fs::Metadata;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

use rayon::prelude::*;
use walkdir::WalkDir;

use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::safety::blocklist::is_blocked_path;
use crate::safety::rules::should_keep_file_with_cutoff;

pub struct WalkRequest {
    pub category: String,
    pub roots: Vec<PathBuf>,
    pub extensions: Option<Vec<String>>,
    pub file_names: Option<Vec<String>>,
    pub risk: Risk,
    pub minimum_age: MinimumAge,
}

impl WalkRequest {
    pub fn files_in(
        category: &str,
        roots: Vec<PathBuf>,
        risk: Risk,
        minimum_age: MinimumAge,
    ) -> Self {
        Self {
            category: category.to_string(),
            roots,
            extensions: None,
            file_names: None,
            risk,
            minimum_age,
        }
    }

    pub fn with_extensions(mut self, extensions: &[&str]) -> Self {
        self.extensions = Some(normalize_extensions(extensions));
        self
    }

    pub fn with_file_names(mut self, names: &[&str]) -> Self {
        self.file_names = Some(names.iter().map(|name| name.to_lowercase()).collect());
        self
    }
}

pub fn collect_files(request: &WalkRequest) -> Vec<JunkItem> {
    let cutoff = request.minimum_age.cutoff();
    request
        .roots
        .par_iter()
        .flat_map(|root| collect_from_root(request, root, cutoff))
        .collect()
}

fn collect_from_root(
    request: &WalkRequest,
    root: &PathBuf,
    cutoff: Option<SystemTime>,
) -> Vec<JunkItem> {
    if let Ok(metadata) = std::fs::symlink_metadata(root) {
        if !metadata.file_type().is_dir() {
            let mut found = Vec::new();
            push_if_match(request, root, &metadata, cutoff, &mut found);
            return found;
        }
    } else {
        return Vec::new();
    }

    let walker = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(keep_entry);

    let mut found = Vec::new();
    for entry in walker.flatten() {
        let file_type = entry.file_type();
        if !file_type.is_file() || file_type.is_symlink() {
            continue;
        }
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };
        if !metadata.is_file() {
            continue;
        }
        push_if_match(request, entry.path(), &metadata, cutoff, &mut found);
    }
    found
}

fn keep_entry(entry: &walkdir::DirEntry) -> bool {
    if entry.depth() == 0 {
        return true;
    }
    if !entry.file_type().is_dir() {
        return true;
    }
    !is_blocked_path(&entry.path().to_string_lossy())
}

fn push_if_match(
    request: &WalkRequest,
    path: &Path,
    metadata: &Metadata,
    cutoff: Option<SystemTime>,
    found: &mut Vec<JunkItem>,
) {
    if !matches_name_filter(request, path) {
        return;
    }

    if should_keep_file_with_cutoff(path, metadata, cutoff) {
        return;
    }

    found.push(JunkItem::new(
        &request.category,
        path.to_path_buf(),
        metadata.len(),
        request.risk,
    ));
}

fn matches_name_filter(request: &WalkRequest, path: &Path) -> bool {
    if request.file_names.is_none() && request.extensions.is_none() {
        return true;
    }

    if let Some(wanted) = &request.file_names
        && let Some(name) = path.file_name()
    {
        let lowered = name.to_string_lossy().to_lowercase();
        if wanted.iter().any(|candidate| candidate == &lowered) {
            return true;
        }
    }

    if let Some(extensions) = &request.extensions
        && let Some(extension) = path.extension()
    {
        let lowered = extension.to_string_lossy().to_lowercase();
        if extensions
            .iter()
            .any(|candidate| extension_matches(candidate, &lowered))
        {
            return true;
        }
    }

    false
}

fn extension_matches(candidate: &str, lowered_extension: &str) -> bool {
    let without_dot = candidate.strip_prefix('.').unwrap_or(candidate);
    without_dot == lowered_extension
}

fn normalize_extensions(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_lowercase()).collect()
}
