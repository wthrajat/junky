use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::drives::all_drive_roots;

use super::cleaner::Cleaner;

pub struct RecycleBinCleaner;

impl Cleaner for RecycleBinCleaner {
    fn id(&self) -> &'static str {
        "recycle-bin"
    }

    fn display_name(&self) -> &'static str {
        "Recycle Bin contents"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        false
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let mut roots = Vec::new();

        for drive in all_drive_roots() {
            let candidate = drive.join("$Recycle.Bin");
            if candidate.is_dir() {
                roots.push(candidate);
            }
        }

        roots.sort();
        roots.dedup();
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age);
        collect_files(&request)
    }
}
