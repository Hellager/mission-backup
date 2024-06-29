//! # Notify
//! 
//! `notify` module contains all configuration about app's notification.

use serde::{Serialize, Deserialize};

/// Configuration for notify
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyConfig {
    /// Whether able to notify
    pub is_granted: bool,

    /// Whether enable notify
    pub enable: bool,

    /// Whether enable create backup notify
    pub when_create: bool,

    /// Whether enable failed backup notify
    pub when_failed: bool
}

impl Default for NotifyConfig {
    fn default() -> Self {
        NotifyConfig {
            is_granted: false,
            enable: false,
            when_create: false,
            when_failed: false
        }
    }
}
