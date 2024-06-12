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

    /// Whether enable mission notify
    pub mission_notify: bool,
}

impl Default for NotifyConfig {
    fn default() -> Self {
        NotifyConfig {
            is_granted: false,
            enable: false,
            mission_notify: false,
        }
    }
}
