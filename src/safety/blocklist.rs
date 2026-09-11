const BLOCKED: &[&str] = &[
    "windows/system32",
    "windows/syswow64",
    "windows/winsxs",
    "windows/installer",
    "windows/servicing",
    "windows/boot",
    "windows/systemapps",
    "softwaredistribution/datastore",
    "catroot2",
    "msocache",
    "package cache",
    ":/recovery",
];

const DISPOSABLE: &[&str] = &[
    "/windows.old/",
    "/$windows.~bt/",
    "/$windows.~ws/",
    "/$sysreset/",
    "/$winreagent/",
    "/esd/",
    "/$getcurrent/",
];

const ALLOWED: &[&str] = &["fntcache.dat"];

pub fn blocked_fragments() -> &'static [&'static str] {
    BLOCKED
}

pub fn disposable_markers() -> &'static [&'static str] {
    DISPOSABLE
}

pub fn allowed_file_names() -> &'static [&'static str] {
    ALLOWED
}

pub fn is_blocked_path(path_text: &str) -> bool {
    if is_allowed_file(path_text) {
        return false;
    }

    if is_inside_disposable(path_text) {
        return false;
    }

    BLOCKED
        .iter()
        .any(|fragment| contains_normalized(path_text, fragment))
}

fn is_allowed_file(path_text: &str) -> bool {
    let bytes = path_text.as_bytes();
    let mut start = 0;
    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b'/' || byte == b'\\' {
            start = index + 1;
        }
    }
    let name = &bytes[start..];
    ALLOWED
        .iter()
        .any(|allowed| equals_ignore_ascii_case(name, allowed.as_bytes()))
}

fn is_inside_disposable(path_text: &str) -> bool {
    DISPOSABLE
        .iter()
        .any(|marker| contains_normalized(path_text, marker))
}

fn contains_normalized(haystack: &str, needle: &str) -> bool {
    let hay = haystack.as_bytes();
    let ndl = needle.as_bytes();
    if ndl.is_empty() {
        return true;
    }
    if ndl.len() > hay.len() {
        return false;
    }
    let first = ndl[0];
    let limit = hay.len() - ndl.len();
    let mut index = 0;
    while index <= limit {
        let mut byte = hay[index];
        if byte == b'\\' {
            byte = b'/';
        }
        if byte.is_ascii_uppercase() {
            byte += 32;
        }
        if byte == first {
            let mut matched = true;
            for offset in 1..ndl.len() {
                let mut other = hay[index + offset];
                if other == b'\\' {
                    other = b'/';
                }
                if other.is_ascii_uppercase() {
                    other += 32;
                }
                if other != ndl[offset] {
                    matched = false;
                    break;
                }
            }
            if matched {
                return true;
            }
        }
        index += 1;
    }
    false
}

fn equals_ignore_ascii_case(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    for (index, &byte) in left.iter().enumerate() {
        let mut normalized = byte;
        if normalized.is_ascii_uppercase() {
            normalized += 32;
        }
        if normalized != right[index] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_blocked_path;

    #[test]
    fn blocks_system32() {
        assert!(is_blocked_path("C:/Windows/System32/drivers/etc"));
    }

    #[test]
    fn blocks_winsxs() {
        assert!(is_blocked_path("C:/Windows/WinSxS/amd64_test"));
    }

    #[test]
    fn allows_user_temp() {
        assert!(!is_blocked_path(
            "C:/Users/Test/AppData/Local/Temp/cache.tmp"
        ));
    }

    #[test]
    fn allows_windows_old_system32() {
        assert!(!is_blocked_path(
            "C:/$Windows.~BT/NewOS/Windows/System32/ntdll.dll"
        ));
    }

    #[test]
    fn allows_fntcache_dat() {
        assert!(!is_blocked_path("C:/Windows/System32/FNTCACHE.DAT"));
    }

    #[test]
    fn blocks_datastore_recovery_and_catroot() {
        assert!(is_blocked_path(
            "C:/Windows/SoftwareDistribution/DataStore/DataStore.edb"
        ));
        assert!(is_blocked_path("C:/Recovery/Logs/bootux.etl"));
        assert!(is_blocked_path("C:/Windows/System32/catroot2/dberr.txt"));
    }

    #[test]
    fn blocks_backslash_variants() {
        assert!(is_blocked_path("C:\\Windows\\System32\\drivers\\etc"));
        assert!(is_blocked_path("C:\\Windows\\WinSxS\\amd64_test"));
    }
}
