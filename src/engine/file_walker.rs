use std::path::PathBuf;
use walkdir::WalkDir;

use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::safety::rules::should_keep_file;

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
    let mut found = Vec::new();

    for root in &request.roots {
        collect_from_root(request, root, &mut found);
    }

    found.sort_by(|left, right| left.path.cmp(&right.path));
    found.dedup_by(|next, current| next.path == current.path);
    found
}

fn collect_from_root(request: &WalkRequest, root: &PathBuf, found: &mut Vec<JunkItem>) {
    if root.is_file() {
        push_if_match(request, root, found);
        return;
    }

    let walker = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| !is_reparse(entry));

    for entry in walker.flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        push_if_match(request, entry.path(), found);
    }
}

fn push_if_match(request: &WalkRequest, path: &std::path::Path, found: &mut Vec<JunkItem>) {
    if !matches_name_filter(request, path) {
        return;
    }

    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return,
    };

    if !metadata.is_file() {
        return;
    }

    if should_keep_file(path, &metadata, request.minimum_age) {
        return;
    }

    found.push(JunkItem::new(
        &request.category,
        path.to_path_buf(),
        metadata.len(),
        request.risk,
    ));
}

fn matches_name_filter(request: &WalkRequest, path: &std::path::Path) -> bool {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    if let Some(wanted) = &request.file_names
        && wanted.contains(&file_name)
    {
        return true;
    }

    if let Some(extensions) = &request.extensions {
        let extension = path
            .extension()
            .map(|value| format!(".{}", value.to_string_lossy().to_lowercase()))
            .unwrap_or_default();
        if extensions.contains(&extension) {
            return true;
        }
    }

    request.file_names.is_none() && request.extensions.is_none()
}

fn normalize_extensions(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_lowercase()).collect()
}

fn is_reparse(entry: &walkdir::DirEntry) -> bool {
    entry
        .path()
        .symlink_metadata()
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
}
