use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::existing_paths;

use super::cleaner::Cleaner;

pub struct UserTempCleaner;

impl Cleaner for UserTempCleaner {
    fn id(&self) -> &'static str {
        "user-temp"
    }

    fn display_name(&self) -> &'static str {
        "User temporary files"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        false
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = existing_paths(&["%TEMP%", "%TMP%", "%LOCALAPPDATA%\\Temp"]);
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age);
        collect_files(&request)
    }
}
