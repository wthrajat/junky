use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::existing_paths;

use super::cleaner::Cleaner;

pub struct DefenderHistoryCleaner;

impl Cleaner for DefenderHistoryCleaner {
    fn id(&self) -> &'static str {
        "defender-history"
    }

    fn display_name(&self) -> &'static str {
        "Defender scan history and logs"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        true
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let history =
            existing_paths(&["%ProgramData%\\Microsoft\\Windows Defender\\Scans\\History"]);
        let support = existing_paths(&["%ProgramData%\\Microsoft\\Windows Defender\\Support"]);

        let mut found = collect_files(&WalkRequest::files_in(
            self.id(),
            history,
            self.risk(),
            minimum_age,
        ));
        found.extend(collect_files(
            &WalkRequest::files_in(self.id(), support, self.risk(), minimum_age)
                .with_extensions(&[".log"]),
        ));
        found.sort_by(|left, right| left.path.cmp(&right.path));
        found.dedup_by(|next, current| next.path == current.path);
        found
    }
}
