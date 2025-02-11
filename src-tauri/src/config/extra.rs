use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtraConfig {
    #[serde(rename = "screensaverEnabled")]
    pub screensaver_enabled: bool,
    #[serde(rename = "screensaverPassword")]
    pub screensaver_password: Option<String>,
    #[serde(rename = "screensaverTimeout")]
    pub screensaver_timeout: u32,   
    #[serde(rename = "monitorDelay")]
    pub monitor_delay: u32,
}

impl Default for ExtraConfig {
    fn default() -> Self {
        Self {
            screensaver_enabled: false,
            screensaver_password: None,
            screensaver_timeout: 15,    // minutes
            monitor_delay: 5,           // seconds  
        }
    }
}
