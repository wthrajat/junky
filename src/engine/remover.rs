use crate::core::clean_summary::CleanSummary;
use crate::core::junk_item::JunkItem;

pub fn remove_items(items: &[JunkItem]) -> CleanSummary {
    let mut summary = CleanSummary::default();

    for item in items {
        remove_single(item, &mut summary);
    }

    summary
}

fn remove_single(item: &JunkItem, summary: &mut CleanSummary) {
    if !item.path.is_file() {
        summary.add_skipped();
        return;
    }

    match std::fs::remove_file(&item.path) {
        Ok(()) => summary.add_removed(item.size_bytes),
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            summary.add_skipped();
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            summary.add_skipped();
        }
        Err(_) => summary.add_failed(),
    }
}
