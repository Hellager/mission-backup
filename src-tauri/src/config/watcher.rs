//! # Watcher
//! 
//! `watcher` module contains all configuration about app's wacher handler related.

use serde::{Serialize, Deserialize};

/// Configuration for watcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatcherConfig {
    /// Watcher timeout in secs
    pub timeout: u64,
}

impl Default for WatcherConfig {
    fn default() -> Self {
        WatcherConfig {
            timeout: 3
        }
    }
}
