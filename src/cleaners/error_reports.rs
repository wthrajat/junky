use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::existing_paths;

use super::cleaner::Cleaner;

pub struct ErrorReportsCleaner;

impl Cleaner for ErrorReportsCleaner {
    fn id(&self) -> &'static str {
        "error-reports"
    }

    fn display_name(&self) -> &'static str {
        "Windows Error Reports"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        false
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = existing_paths(&[
            "C:\\ProgramData\\Microsoft\\Windows\\WER\\ReportQueue",
            "C:\\ProgramData\\Microsoft\\Windows\\WER\\ReportArchive",
            "%LOCALAPPDATA%\\Microsoft\\Windows\\WER\\ReportQueue",
            "%LOCALAPPDATA%\\Microsoft\\Windows\\WER\\ReportArchive",
        ]);
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age);
        collect_files(&request)
    }
}
