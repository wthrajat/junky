use std::env;
use std::path::PathBuf;

fn test_root() -> Option<PathBuf> {
    env::var_os("JUNKY_TEST_ROOT").map(PathBuf::from)
}

fn windows_value(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

pub fn expand_windows_path(raw: &str) -> PathBuf {
    let system_drive = windows_value("SystemDrive");
    let system_root = windows_value("SystemRoot");
    let temp = windows_value("TEMP");
    let tmp = windows_value("TMP");
    let local_app_data = windows_value("LOCALAPPDATA");
    let app_data = windows_value("APPDATA");
    let user_profile = windows_value("USERPROFILE");
    let program_data = windows_value("ProgramData");
    let public = windows_value("PUBLIC");

    let mut expanded = raw.to_string();

    if let Some(real) = system_drive {
        expanded = expanded.replace("%SystemDrive%", &real);
    }
    if let Some(real) = system_root {
        expanded = expanded.replace("%SystemRoot%", &real);
        expanded = expanded.replace("%WinDir%", &real);
        expanded = expanded.replace("%WINDIR%", &real);
        expanded = expanded.replace("%windir%", &real);
    }
    if let Some(real) = temp {
        expanded = expanded.replace("%TEMP%", &real);
        expanded = expanded.replace("%Temp%", &real);
    }
    if let Some(real) = tmp {
        expanded = expanded.replace("%TMP%", &real);
        expanded = expanded.replace("%Tmp%", &real);
    }
    if let Some(real) = local_app_data {
        expanded = expanded.replace("%LOCALAPPDATA%", &real);
        expanded = expanded.replace("%LocalAppData%", &real);
    }
    if let Some(real) = app_data {
        expanded = expanded.replace("%APPDATA%", &real);
        expanded = expanded.replace("%AppData%", &real);
    }
    if let Some(real) = user_profile {
        expanded = expanded.replace("%USERPROFILE%", &real);
        expanded = expanded.replace("%UserProfile%", &real);
    }
    if let Some(real) = program_data {
        expanded = expanded.replace("%ProgramData%", &real);
        expanded = expanded.replace("%CommonAppData%", &real);
    }
    if let Some(real) = public {
        expanded = expanded.replace("%Public%", &real);
        expanded = expanded.replace("%PUBLIC%", &real);
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
