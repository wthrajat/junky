pub fn is_elevated() -> bool {
    #[cfg(windows)]
    {
        windows_elevation()
    }

    #[cfg(not(windows))]
    {
        false
    }
}

#[cfg(windows)]
fn windows_elevation() -> bool {
    std::env::var("JUNKY_ASSUME_ELEVATED")
        .map(|value| value == "1")
        .unwrap_or(false)
}
