//! Lumina Asset - Asset management system

use std::path::Path;
use std::sync::Arc;

/// Asset engine
pub struct AssetEngine;

impl AssetEngine {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AssetEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Asset handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetHandle(pub u64);

impl Default for AssetHandle {
    fn default() -> Self {
        AssetHandle(0)
    }
}
