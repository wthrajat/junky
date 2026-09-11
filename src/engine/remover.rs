use rayon::prelude::*;

use crate::core::clean_summary::CleanSummary;
use crate::core::junk_item::JunkItem;

pub fn remove_items(items: &[JunkItem]) -> CleanSummary {
    let (removed_files, freed_bytes, skipped_files, failed_files) = items
        .par_iter()
        .map(|item| match std::fs::remove_file(&item.path) {
            Ok(()) => (1_usize, item.size_bytes, 0_usize, 0_usize),
            Err(error)
                if error.kind() == std::io::ErrorKind::NotFound
                    || error.kind() == std::io::ErrorKind::PermissionDenied
                    || error.kind() == std::io::ErrorKind::IsADirectory =>
            {
                (0, 0, 1, 0)
            }
            Err(_) => (0, 0, 0, 1),
        })
        .reduce(
            || (0_usize, 0_u64, 0_usize, 0_usize),
            |left, right| {
                (
                    left.0 + right.0,
                    left.1 + right.1,
                    left.2 + right.2,
                    left.3 + right.3,
                )
            },
        );
    CleanSummary {
        removed_files,
        freed_bytes,
        skipped_files,
        failed_files,
    }
}
