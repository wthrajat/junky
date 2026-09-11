use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::expand_windows_path;

use super::super::cleaner::Cleaner;
use super::shared::chromium_profiles;
use super::shared::existing_dirs;
use super::shared::existing_subdirs;

pub struct EdgeCleaner;

impl Cleaner for EdgeCleaner {
    fn id(&self) -> &'static str {
        "browser-edge"
    }

    fn display_name(&self) -> &'static str {
        "Edge cache"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        false
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let base = expand_windows_path("%LOCALAPPDATA%\\Microsoft\\Edge\\User Data");
        let profiles = chromium_profiles(&base);
        let mut roots = existing_subdirs(
            &profiles,
            &[
                "Cache",
                "Code Cache",
                "GPUCache",
                "Media Cache",
                "ShaderCache",
            ],
        );
        roots.extend(existing_dirs(&[
            base.join("ShaderCache"),
            base.join("GrShaderCache"),
            base.join("GraphiteDawnCache"),
        ]));

        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age);
        collect_files(&request)
    }
}
