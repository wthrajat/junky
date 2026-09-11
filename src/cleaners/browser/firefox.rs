use std::fs;

use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::expand_windows_path;

use super::super::cleaner::Cleaner;

pub struct FirefoxCleaner;

impl Cleaner for FirefoxCleaner {
    fn id(&self) -> &'static str {
        "browser-firefox"
    }

    fn display_name(&self) -> &'static str {
        "Firefox cache"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        false
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = firefox_cache_roots();
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age);
        collect_files(&request)
    }
}

fn firefox_cache_roots() -> Vec<std::path::PathBuf> {
    let profiles = expand_windows_path("%LOCALAPPDATA%\\Mozilla\\Firefox\\Profiles");
    let mut roots = Vec::new();

    if let Ok(entries) = fs::read_dir(&profiles) {
        for entry in entries.flatten() {
            if !entry.path().is_dir() {
                continue;
            }
            for name in ["cache2", "shader-cache", "startupCache"] {
                let candidate = entry.path().join(name);
                if candidate.is_dir() {
                    roots.push(candidate);
                }
            }
        }
    }

    roots.sort();
    roots.dedup();
    roots
}
