use rayon::prelude::*;
use serde::Serialize;
use std::collections::BTreeMap;

use super::junk_item::JunkItem;

#[derive(Debug, Default, Serialize)]
pub struct CategoryTotal {
    pub files: usize,
    pub bytes: u64,
}

#[derive(Debug, Default, Serialize)]
pub struct ScanSummary {
    pub files: usize,
    pub bytes: u64,
    pub by_category: BTreeMap<String, CategoryTotal>,
    pub items: Vec<JunkItem>,
}

impl ScanSummary {
    pub fn from_items(mut items: Vec<JunkItem>) -> Self {
        items.par_sort_unstable_by(|left, right| {
            left.category
                .cmp(&right.category)
                .then(left.path.cmp(&right.path))
        });

        let mut summary = Self {
            files: items.len(),
            bytes: 0,
            by_category: BTreeMap::new(),
            items: Vec::with_capacity(items.len()),
        };

        for item in items {
            summary.bytes += item.size_bytes;
            let total = summary
                .by_category
                .entry(item.category.clone())
                .or_default();
            total.files += 1;
            total.bytes += item.size_bytes;
            summary.items.push(item);
        }

        summary
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
