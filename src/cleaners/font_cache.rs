use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::existing_paths;

use super::cleaner::Cleaner;

pub struct FontCacheCleaner;

impl Cleaner for FontCacheCleaner {
    fn id(&self) -> &'static str {
        "font-cache"
    }

    fn display_name(&self) -> &'static str {
        "Font cache"
    }

    fn risk(&self) -> Risk {
        Risk::Aggressive
    }

    fn needs_admin(&self) -> bool {
        true
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = existing_paths(&[
            "%SystemRoot%\\ServiceProfiles\\LocalService\\AppData\\Local\\FontCache",
            "%SystemRoot%\\System32\\FNTCACHE.DAT",
        ]);
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age);
        collect_files(&request)
    }
}
