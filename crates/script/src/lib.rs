//! Lumina Script - Scripting system with hot reload support

use std::collections::HashMap;

/// Script engine
pub struct ScriptEngine;

impl ScriptEngine {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}
