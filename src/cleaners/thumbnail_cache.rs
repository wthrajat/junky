use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::existing_paths;

use super::cleaner::Cleaner;

pub struct ThumbnailCacheCleaner;

impl Cleaner for ThumbnailCacheCleaner {
    fn id(&self) -> &'static str {
        "thumbnail-cache"
    }

    fn display_name(&self) -> &'static str {
        "Thumbnail cache"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        false
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = existing_paths(&["%LOCALAPPDATA%\\Microsoft\\Windows\\Explorer"]);
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age)
            .with_extensions(&[".db"]);
        collect_files(&request)
            .into_iter()
            .filter(thumbnail_name)
            .collect()
    }
}

fn thumbnail_name(item: &JunkItem) -> bool {
    item.path
        .file_name()
        .map(|name| name.to_string_lossy().to_lowercase())
        .map(|name| name.starts_with("thumbcache_"))
        .unwrap_or(false)
}
