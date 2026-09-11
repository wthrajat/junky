use std::fs;
use std::path::PathBuf;

pub fn chromium_profiles(base: &PathBuf) -> Vec<PathBuf> {
    let mut profiles = Vec::new();
    let default_profile = base.join("Default");
    if default_profile.is_dir() {
        profiles.push(default_profile);
    }

    if let Ok(entries) = fs::read_dir(base) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("Profile ") && entry.path().is_dir() {
                profiles.push(entry.path());
            }
        }
    }

    profiles.sort();
    profiles.dedup();
    profiles
}

pub fn existing_subdirs(parents: &[PathBuf], names: &[&str]) -> Vec<PathBuf> {
    let mut found = Vec::new();

    for parent in parents {
        for name in names {
            let candidate = parent.join(name);
            if candidate.is_dir() {
                found.push(candidate);
            }
        }
    }

    found.sort();
    found.dedup();
    found
}

pub fn existing_dirs(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut found = Vec::new();

    for path in paths {
        if path.is_dir() {
            found.push(path.clone());
        }
    }

    found.sort();
    found.dedup();
    found
}
