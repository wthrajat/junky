pub fn blocked_fragments() -> &'static [&'static str] {
    &[
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
    ]
}

pub fn disposable_markers() -> &'static [&'static str] {
    &[
        "/windows.old/",
        "/$windows.~bt/",
        "/$windows.~ws/",
        "/$sysreset/",
        "/$winreagent/",
        "/esd/",
        "/$getcurrent/",
    ]
}

pub fn allowed_file_names() -> &'static [&'static str] {
    &["fntcache.dat"]
}

pub fn is_blocked_path(path_text: &str) -> bool {
    let lowered = path_text.to_lowercase().replace('\\', "/");

    if is_allowed_file(&lowered) {
        return false;
    }

    if is_inside_disposable(&lowered) {
        return false;
    }

    blocked_fragments()
        .iter()
        .any(|fragment| lowered.contains(fragment))
}

fn is_allowed_file(lowered_path: &str) -> bool {
    let file_name = lowered_path.rsplit('/').next().unwrap_or("");

    allowed_file_names().contains(&file_name)
}

fn is_inside_disposable(lowered_path: &str) -> bool {
    disposable_markers()
        .iter()
        .any(|marker| lowered_path.contains(marker))
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
}
