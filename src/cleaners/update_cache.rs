use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::existing_paths;

use super::cleaner::Cleaner;

pub struct UpdateCacheCleaner;

impl Cleaner for UpdateCacheCleaner {
    fn id(&self) -> &'static str {
        "update-cache"
    }

    fn display_name(&self) -> &'static str {
        "Windows Update download cache"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        true
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = existing_paths(&[
            "%SystemRoot%\\SoftwareDistribution\\Download",
            "C:\\Windows\\SoftwareDistribution\\Download",
        ]);
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age);
        collect_files(&request)
    }
}
