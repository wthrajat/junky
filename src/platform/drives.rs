use std::path::PathBuf;

use super::environment::expand_windows_path;

pub fn system_drive_roots(extra: &[&str]) -> Vec<PathBuf> {
    let mut roots = Vec::new();

    for raw in extra {
        let path = expand_windows_path(raw);
        if path.is_dir() {
            roots.push(path);
        }
    }

    roots.sort();
    roots.dedup();
    roots
}

pub fn all_drive_roots() -> Vec<PathBuf> {
    #[cfg(windows)]
    {
        windows_drives()
    }

    #[cfg(not(windows))]
    {
        unix_drive_fallback()
    }
}

#[cfg(windows)]
fn windows_drives() -> Vec<PathBuf> {
    let mut drives = Vec::new();

    for letter in b'A'..=b'Z' {
        let candidate = format!("{}:/", letter as char);
        let path = PathBuf::from(&candidate);
        if path.is_dir() {
            drives.push(path);
        }
    }

    drives
}

#[cfg(not(windows))]
fn unix_drive_fallback() -> Vec<PathBuf> {
    if let Ok(root) = std::env::var("JUNKY_TEST_ROOT") {
        return vec![PathBuf::from(root)];
    }

    vec![PathBuf::from("/")]
}
