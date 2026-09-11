fn main() {
    slint_build::compile("ui/app.slint").unwrap();
    #[cfg(target_os = "windows")]
    embed_icon();
}

#[cfg(target_os = "windows")]
fn embed_icon() {
    let mut resource = winresource::WindowsResource::new();
    resource.set_icon("assets/icon.ico");
    resource.compile().unwrap();
}
