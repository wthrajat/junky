use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct CleanSummary {
    pub removed_files: usize,
    pub freed_bytes: u64,
    pub skipped_files: usize,
    pub failed_files: usize,
}

impl CleanSummary {
    pub fn add_removed(&mut self, size_bytes: u64) {
        self.removed_files += 1;
        self.freed_bytes += size_bytes;
    }

    pub fn add_skipped(&mut self) {
        self.skipped_files += 1;
    }

    pub fn add_failed(&mut self) {
        self.failed_files += 1;
    }
}
