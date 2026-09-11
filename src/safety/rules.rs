use std::fs::Metadata;
use std::path::Path;
use std::time::SystemTime;

use super::blocklist::is_blocked_path;
use crate::core::minimum_age::MinimumAge;

pub fn should_keep_file(path: &Path, metadata: &Metadata, minimum_age: MinimumAge) -> bool {
    should_keep_file_with_cutoff(path, metadata, minimum_age.cutoff())
}

pub fn should_keep_file_with_cutoff(
    path: &Path,
    metadata: &Metadata,
    cutoff: Option<SystemTime>,
) -> bool {
    if is_blocked_path(&path.to_string_lossy()) {
        return true;
    }

    if !is_old_enough_with_cutoff(metadata, cutoff) {
        return true;
    }

    false
}

fn is_old_enough_with_cutoff(metadata: &Metadata, cutoff: Option<SystemTime>) -> bool {
    let limit = match cutoff {
        Some(limit) => limit,
        None => return true,
    };

    let modified = match metadata.modified() {
        Ok(modified) => modified,
        Err(_) => return false,
    };

    modified <= limit
}
