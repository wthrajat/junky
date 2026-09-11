use crate::cleaners::CleanerInfo;
use crate::core::clean_summary::CleanSummary;
use crate::core::scan_summary::ScanSummary;
use crate::core::size_format::format_bytes;

pub fn print_scan(summary: &ScanSummary, as_json: bool, limit: usize) {
    if as_json {
        print_json(summary);
        return;
    }

    print_scan_human(summary, limit);
}

pub fn print_clean(scanned: &ScanSummary, cleaned: &CleanSummary, as_json: bool) {
    if as_json {
        print_clean_json(scanned, cleaned);
        return;
    }

    print_clean_human(scanned, cleaned);
}

pub fn print_list(infos: &[CleanerInfo], as_json: bool) {
    if as_json {
        print_json(infos);
        return;
    }

    println!("Available cleaners:");
    for info in infos {
        let admin = if info.needs_admin { "admin" } else { "user" };
        println!(
            "  {:22} {:32} {:11} {}",
            info.id,
            info.display_name,
            info.risk.label(),
            admin
        );
    }
}

fn print_scan_human(summary: &ScanSummary, limit: usize) {
    println!(
        "Found {} files using {}",
        summary.files,
        format_bytes(summary.bytes)
    );

    for (category, total) in &summary.by_category {
        println!(
            "  {:22} {:5} files  {}",
            category,
            total.files,
            format_bytes(total.bytes)
        );
    }

    if summary.is_empty() {
        return;
    }

    println!();
    println!("Top files:");
    for item in summary.items.iter().take(limit) {
        println!(
            "  {}  {}",
            format_bytes(item.size_bytes),
            item.path.display()
        );
    }

    if summary.files > limit {
        println!("  ... and {} more", summary.files - limit);
    }

    println!();
    println!("Run with `clean --yes` to delete the files found above.");
}

fn print_clean_human(scanned: &ScanSummary, cleaned: &CleanSummary) {
    println!(
        "Scanned {} files using {}",
        scanned.files,
        format_bytes(scanned.bytes)
    );
    println!(
        "Removed {} files freeing {}",
        cleaned.removed_files,
        format_bytes(cleaned.freed_bytes)
    );
    println!(
        "Skipped {}  Failed {}",
        cleaned.skipped_files, cleaned.failed_files
    );
}

fn print_clean_json(scanned: &ScanSummary, cleaned: &CleanSummary) {
    let payload = serde_json::json!({
        "scanned_files": scanned.files,
        "scanned_bytes": scanned.bytes,
        "removed_files": cleaned.removed_files,
        "freed_bytes": cleaned.freed_bytes,
        "skipped_files": cleaned.skipped_files,
        "failed_files": cleaned.failed_files,
    });
    print_json(&payload);
}

fn print_json(value: &(impl serde::Serialize + ?Sized)) {
    match serde_json::to_string_pretty(value) {
        Ok(text) => println!("{text}"),
        Err(_) => println!("{{}}"),
    }
}
