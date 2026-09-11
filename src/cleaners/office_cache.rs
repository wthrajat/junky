use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::existing_paths;

use super::cleaner::Cleaner;

pub struct OfficeCacheCleaner;

impl Cleaner for OfficeCacheCleaner {
    fn id(&self) -> &'static str {
        "office-cache"
    }

    fn display_name(&self) -> &'static str {
        "Office file cache"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        false
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = existing_paths(&[
            "%LOCALAPPDATA%\\Microsoft\\Office\\16.0\\OfficeFileCache",
            "%LOCALAPPDATA%\\Microsoft\\Office\\15.0\\OfficeFileCache",
            "%LOCALAPPDATA%\\Microsoft\\Office\\OffDiag",
        ]);
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age);
        collect_files(&request)
    }
}
