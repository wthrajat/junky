use std::env;
use std::path::PathBuf;

fn test_root() -> Option<PathBuf> {
    env::var_os("JUNKY_TEST_ROOT").map(PathBuf::from)
}

fn windows_value(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

pub fn expand_windows_path(raw: &str) -> PathBuf {
    let mut expanded = raw.to_string();

    let replacements = [
        ("%SystemDrive%", windows_value("SystemDrive")),
        ("%SystemRoot%", windows_value("SystemRoot")),
        ("%WinDir%", windows_value("SystemRoot")),
        ("%WINDIR%", windows_value("SystemRoot")),
        ("%windir%", windows_value("SystemRoot")),
        ("%TEMP%", windows_value("TEMP")),
        ("%Temp%", windows_value("TEMP")),
        ("%TMP%", windows_value("TMP")),
        ("%Tmp%", windows_value("TMP")),
        ("%LOCALAPPDATA%", windows_value("LOCALAPPDATA")),
        ("%LocalAppData%", windows_value("LOCALAPPDATA")),
        ("%APPDATA%", windows_value("APPDATA")),
        ("%AppData%", windows_value("APPDATA")),
        ("%USERPROFILE%", windows_value("USERPROFILE")),
        ("%UserProfile%", windows_value("USERPROFILE")),
        ("%ProgramData%", windows_value("ProgramData")),
        ("%CommonAppData%", windows_value("ProgramData")),
        ("%Public%", windows_value("PUBLIC")),
        ("%PUBLIC%", windows_value("PUBLIC")),
    ];

    for (token, value) in replacements {
        if let Some(real) = value {
            expanded = expanded.replace(token, &real);
        }
    }

    let with_separators = expanded.replace('\\', "/");

    if let Some(root) = test_root()
        && let Some(stripped) = with_separators
            .strip_prefix("C:/")
            .or_else(|| with_separators.strip_prefix("c:/"))
    {
        return root.join(stripped);
    }

    PathBuf::from(with_separators)
}

pub fn existing_paths(raw_paths: &[&str]) -> Vec<PathBuf> {
    let mut found = Vec::new();

    for raw in raw_paths {
        let path = expand_windows_path(raw);
        if path.is_dir() || path.is_file() {
            found.push(path);
        }
    }

    found.sort();
    found.dedup();
    found
}
