use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::existing_paths;

use super::cleaner::Cleaner;

pub struct InternetCacheCleaner;

impl Cleaner for InternetCacheCleaner {
    fn id(&self) -> &'static str {
        "internet-cache"
    }

    fn display_name(&self) -> &'static str {
        "Temporary Internet files"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        false
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = existing_paths(&[
            "%LOCALAPPDATA%\\Microsoft\\Windows\\INetCache",
            "%LOCALAPPDATA%\\Microsoft\\Windows\\WebCache",
            "%LOCALAPPDATA%\\Microsoft\\Windows\\Temporary Internet Files",
        ]);
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age);
        collect_files(&request)
    }
}
