use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;
use crate::engine::file_walker::WalkRequest;
use crate::engine::file_walker::collect_files;
use crate::platform::environment::existing_paths;

use super::cleaner::Cleaner;

pub struct DeliveryOptimizationCleaner;

impl Cleaner for DeliveryOptimizationCleaner {
    fn id(&self) -> &'static str {
        "delivery-optimization"
    }

    fn display_name(&self) -> &'static str {
        "Delivery Optimization cache"
    }

    fn risk(&self) -> Risk {
        Risk::Safe
    }

    fn needs_admin(&self) -> bool {
        true
    }

    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem> {
        let roots = existing_paths(&[
            "%SystemRoot%\\SoftwareDistribution\\DeliveryOptimization",
            "C:\\Windows\\SoftwareDistribution\\DeliveryOptimization",
            "%SystemRoot%\\ServiceProfiles\\NetworkService\\AppData\\Local\\Microsoft\\Windows\\DeliveryOptimization",
            "%ProgramData%\\Microsoft\\Windows\\DeliveryOptimization\\Cache",
            "%SystemRoot%\\DeliveryOptimization",
        ]);
        let request = WalkRequest::files_in(self.id(), roots, self.risk(), minimum_age);
        collect_files(&request)
    }
}
