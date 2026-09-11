use serde::Serialize;

use crate::core::junk_item::JunkItem;
use crate::core::minimum_age::MinimumAge;
use crate::core::risk::Risk;

pub trait Cleaner: Send + Sync {
    fn id(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    fn risk(&self) -> Risk;
    fn needs_admin(&self) -> bool;
    fn scan(&self, minimum_age: MinimumAge) -> Vec<JunkItem>;
}

#[derive(Debug, Clone, Serialize)]
pub struct CleanerInfo {
    pub id: String,
    pub display_name: String,
    pub risk: Risk,
    pub needs_admin: bool,
}
