use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::existing_paths;

use super::cleaner::Cleaner;

pub struct ShaderCacheCleaner;

impl Cleaner for ShaderCacheCleaner {
    fn id(&self) -> &'static str {
        "shader-cache"
    }

    fn display_name(&self) -> &'static str {
        "DirectX and driver shader caches"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        false
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = existing_paths(&[
            "%LOCALAPPDATA%\\D3DSCache",
            "%LOCALAPPDATA%\\AMD\\DxCache",
            "%LOCALAPPDATA%\\NVIDIA\\DXCache",
            "%LOCALAPPDATA%\\NVIDIA\\GLCache",
            "%LOCALAPPDATA%\\NVIDIA Corporation\\NV_Cache",
            "%USERPROFILE%\\AppData\\LocalLow\\Intel\\ShaderCache",
        ]);
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age);
        collect_files(&request)
    }
}
