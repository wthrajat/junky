use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::drives::all_drive_roots;

use super::cleaner::Cleaner;

pub struct ExtensionSweepCleaner;

impl Cleaner for ExtensionSweepCleaner {
    fn id(&self) -> &'static str {
        "extension-sweep"
    }

    fn display_name(&self) -> &'static str {
        "Stray temporary extensions"
    }

    fn risk(&self) -> Risk {
        Risk::Aggressive
    }

    fn needs_admin(&self) -> bool {
        false
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = all_drive_roots();
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age)
            .with_extensions(&[
                ".tmp", ".temp", ".bak", ".old", ".wbk", ".xlk", ".gid", ".chk", ".syd", ".$$$",
                ".@@@",
            ])
            .with_file_names(&["thumbs.db"]);
        collect_files(&request)
    }
}
