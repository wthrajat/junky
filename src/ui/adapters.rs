use std::collections::HashMap;

use crate::cleaners::catalog::all_cleaners;
use crate::cleaners::catalog::describe_all;
use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::scan_summary::ScanSummary;
use crate::core::size_format::format_bytes;
use crate::engine::remover::remove_items;
use crate::engine::scanner::scan_all;

pub struct CategoryData {
    pub id: String,
    pub label: String,
    pub detail: String,
    pub aggressive: bool,
    pub checked: bool,
}

pub fn initial_categories() -> (Vec<CategoryData>, Vec<CategoryData>) {
    let mut safe = Vec::new();
    let mut aggressive = Vec::new();

    for info in describe_all() {
        let entry = CategoryData {
            label: info.display_name.clone(),
            detail: category_detail(&info.id, info.needs_admin),
            id: info.id.clone(),
            aggressive: info.risk.is_aggressive(),
            checked: !info.risk.is_aggressive(),
        };
        if entry.aggressive {
            aggressive.push(entry);
        } else {
            safe.push(entry);
        }
    }

    (safe, aggressive)
}

pub fn minimum_age_for_label(label: &str) -> MinimumAge {
    match label {
        "Last 24 hours" => MinimumAge::from_hours(24),
        "Last 7 days" => MinimumAge::from_hours(168),
        _ => MinimumAge::none(),
    }
}

pub fn run_scan(checked_ids: &[String], age: MinimumAge) -> ScanSummary {
    let cleaners: Vec<_> = all_cleaners(true)
        .into_iter()
        .filter(|cleaner| checked_ids.iter().any(|id| id.as_str() == cleaner.id()))
        .collect();

    scan_all(&cleaners, age)
}

pub fn remove_selection(snapshot: &[JunkItem], selected: &[bool]) -> crate::core::CleanSummary {
    let targets: Vec<JunkItem> = snapshot
        .iter()
        .zip(selected.iter())
        .filter(|(_, on)| **on)
        .map(|(item, _)| item.clone())
        .collect();

    remove_items(&targets)
}

pub fn category_sizes(summary: &ScanSummary) -> HashMap<String, String> {
    summary
        .by_category
        .iter()
        .map(|(name, total)| (name.clone(), format_bytes(total.bytes)))
        .collect()
}

pub fn totals_text(summary: &ScanSummary) -> (String, String) {
    (format_bytes(summary.bytes), summary.files.to_string())
}

pub fn selection_summary(snapshot: &[JunkItem], selected: &[bool]) -> (usize, u64) {
    let mut count = 0;
    let mut bytes = 0;

    for (item, on) in snapshot.iter().zip(selected.iter()) {
        if *on {
            count += 1;
            bytes += item.size_bytes;
        }
    }

    (count, bytes)
}

pub fn confirm_detail(count: usize, bytes: u64) -> String {
    format!(
        "{count} files using {size} will be permanently deleted.",
        size = format_bytes(bytes)
    )
}

pub fn clean_status(removed: usize, freed: u64, skipped: usize, failed: usize) -> String {
    format!(
        "Removed {removed} files ({size}) · skipped {skipped} locked · {failed} failed.",
        size = format_bytes(freed)
    )
}

fn category_detail(id: &str, needs_admin: bool) -> String {
    let base = match id {
        "user-temp" => "Your temporary files",
        "system-temp" => "Windows temporary files",
        "update-cache" => "Downloaded update payloads",
        "delivery-optimization" => "Shared update cache",
        "recycle-bin" => "Deleted files awaiting purge",
        "thumbnail-cache" => "Explorer thumbnail database",
        "icon-cache" => "Explorer icon database",
        "internet-cache" => "Temporary internet files",
        "error-reports" => "Crash report queue",
        "crash-dumps" => "Memory and kernel dumps",
        "shader-cache" => "GPU driver shader blobs",
        "office-cache" => "Office sync leftovers",
        "app-caches" => "Discord, Slack, VS Code, Teams",
        "defender-history" => "Scan history and logs",
        "windows-logs" => "Setup and servicing logs",
        "browser-edge" => "Edge web cache only",
        "browser-chrome" => "Chrome web cache only",
        "browser-firefox" => "Firefox web cache only",
        "browser-brave" => "Brave web cache only",
        "prefetch" => "Launch prefetch traces",
        "windows-old" => "Prior installs and upgrade scraps",
        "extension-sweep" => "Stray temp extensions, all drives",
        "font-cache" => "Font rasterizer cache",
        _ => "Cached files",
    };

    if needs_admin {
        format!("{base} · needs admin")
    } else {
        base.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::confirm_detail;
    use super::minimum_age_for_label;

    #[test]
    fn maps_age_labels() {
        assert!(minimum_age_for_label("Any age").as_duration().is_none());
        assert!(
            minimum_age_for_label("Last 24 hours")
                .as_duration()
                .is_some()
        );
    }

    #[test]
    fn describes_confirmation() {
        let text = confirm_detail(3, 2048);
        assert!(text.contains('3'));
    }
}
