use std::fs::Metadata;
use std::path::Path;
use std::time::SystemTime;

use super::blocklist::is_blocked_path;
use crate::core::minimum_age::MinimumAge;

pub fn should_keep_file(path: &Path, metadata: &Metadata, minimum_age: MinimumAge) -> bool {
    if is_blocked_path(&path.to_string_lossy()) {
        return true;
    }

    if !is_old_enough(metadata, minimum_age) {
        return true;
    }

    false
}

fn is_old_enough(metadata: &Metadata, minimum_age: MinimumAge) -> bool {
    let required = match minimum_age.as_duration() {
        Some(required) => required,
        None => return true,
    };

    let modified = match metadata.modified() {
        Ok(modified) => modified,
        Err(_) => return false,
    };

    SystemTime::now()
        .duration_since(modified)
        .map(|age| age >= required)
        .unwrap_or(false)
}
