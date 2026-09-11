pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = &UNITS[0];

    for next in UNITS.iter().skip(1) {
        if value < 1024.0 {
            break;
        }
        value /= 1024.0;
        unit = next;
    }

    if unit == &UNITS[0] {
        format!("{} {}", bytes, unit)
    } else {
        format!("{:.2} {}", value, unit)
    }
}

#[cfg(test)]
mod tests {
    use super::format_bytes;

    #[test]
    fn formats_bytes_without_decimals() {
        assert_eq!(format_bytes(512), "512 B");
    }

    #[test]
    fn formats_kilobytes_with_decimals() {
        assert_eq!(format_bytes(2048), "2.00 KB");
    }

    #[test]
    fn formats_megabytes_with_decimals() {
        assert_eq!(format_bytes(5 * 1024 * 1024), "5.00 MB");
    }
}
