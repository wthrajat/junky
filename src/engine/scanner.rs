use std::collections::HashMap;
use std::collections::hash_map::Entry;

use rayon::prelude::*;

use crate::cleaners::cleaner::Cleaner;
use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::scan_summary::ScanSummary;

pub fn scan_all(cleaners: &[Box<dyn Cleaner>], minimum_age: MinimumAge) -> ScanSummary {
    let items: Vec<JunkItem> = cleaners
        .par_iter()
        .flat_map(|cleaner| cleaner.scan(minimum_age))
        .collect();

    ScanSummary::from_items(dedup_items(items))
}

fn dedup_items(items: Vec<JunkItem>) -> Vec<JunkItem> {
    let mut by_path: HashMap<std::path::PathBuf, JunkItem> =
        HashMap::with_capacity(items.len().min(1 << 20));
    for item in items {
        match by_path.entry(item.path.clone()) {
            Entry::Occupied(mut slot) => {
                if slot.get().risk.is_aggressive() && !item.risk.is_aggressive() {
                    slot.insert(item);
                }
            }
            Entry::Vacant(slot) => {
                slot.insert(item);
            }
        }
    }
    by_path.into_values().collect()
}
