use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::existing_paths;

use super::cleaner::Cleaner;

pub struct PrefetchCleaner;

impl Cleaner for PrefetchCleaner {
    fn id(&self) -> &'static str {
        "prefetch"
    }

    fn display_name(&self) -> &'static str {
        "Prefetch data"
    }

    fn risk(&self) -> Risk {
        Risk::Aggressive
    }

    fn needs_admin(&self) -> bool {
        true
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = existing_paths(&["C:\\Windows\\Prefetch"]);
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age)
            .with_extensions(&[".pf"]);
        collect_files(&request)
    }
}
