use serde::Serialize;
use std::path::PathBuf;

use super::risk::Risk;

#[derive(Debug, Clone, Serialize)]
pub struct JunkItem {
    pub category: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub risk: Risk,
}

impl JunkItem {
    pub fn new(category: &str, path: PathBuf, size_bytes: u64, risk: Risk) -> Self {
        Self {
            category: category.to_string(),
            path,
            size_bytes,
            risk,
        }
    }
}
