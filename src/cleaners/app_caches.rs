use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::existing_paths;

use super::cleaner::Cleaner;

pub struct AppCachesCleaner;

impl Cleaner for AppCachesCleaner {
    fn id(&self) -> &'static str {
        "app-caches"
    }

    fn display_name(&self) -> &'static str {
        "App caches"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        false
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = existing_paths(&[
            "%APPDATA%\\Discord\\Cache",
            "%APPDATA%\\Discord\\Code Cache",
            "%APPDATA%\\Discord\\GPUCache",
            "%APPDATA%\\Slack\\Cache",
            "%APPDATA%\\Slack\\Code Cache",
            "%APPDATA%\\Slack\\GPUCache",
            "%APPDATA%\\Code\\Cache",
            "%APPDATA%\\Code\\Code Cache",
            "%APPDATA%\\Code\\GPUCache",
            "%APPDATA%\\Microsoft\\Teams\\Cache",
            "%APPDATA%\\Microsoft\\Teams\\GPUCache",
            "%APPDATA%\\Microsoft\\Teams\\blob_storage",
            "%APPDATA%\\Microsoft\\Teams\\tmp",
        ]);
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age);
        collect_files(&request)
    }
}
