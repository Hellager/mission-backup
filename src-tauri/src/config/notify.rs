use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyConfig {
    #[serde(rename = "isGranted")]
    pub is_granted: bool,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "triggerNotify")]
    pub trigger_notify: bool
}

impl Default for NotifyConfig {
    fn default() -> Self {
        NotifyConfig {
            is_granted: false,
            is_enabled: false,
            trigger_notify: true
        }
    }
}
