use rayon::prelude::*;

use crate::cleaners::cleaner::Cleaner;
use crate::core::minimum_age::MinimumAge;
use crate::core::scan_summary::ScanSummary;

pub fn scan_all(cleaners: &[Box<dyn Cleaner>], minimum_age: MinimumAge) -> ScanSummary {
    let mut items: Vec<_> = cleaners
        .par_iter()
        .flat_map(|cleaner| cleaner.scan(minimum_age))
        .collect();

    items.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.risk.is_aggressive().cmp(&right.risk.is_aggressive()))
    });
    items.dedup_by(|next, current| next.path == current.path);

    ScanSummary::from_items(items)
}
