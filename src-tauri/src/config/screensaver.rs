//! # Screen
//! 
//! `screen` module contains all configuration about app's screen related.

use serde::{Serialize, Deserialize};

/// Configuration for screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreensaverConfig {
    /// Whether enable screensaver
    pub enable: bool,

    /// Password for screensaver
    pub password: String,

    /// Whether has been locked
    pub is_locked: bool,
}

impl Default for ScreensaverConfig {
    fn default() -> Self {
        ScreensaverConfig {
            enable: false,
            password: "".to_string(),
            is_locked: false,
        }
    }
}
