use crate::utils::common::get_sys_locale;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub theme: Theme,
    #[serde(rename = "themeOption")]
    pub theme_option: ThemeOption,
    #[serde(rename = "autoStart")]
    pub auto_start: bool,
    #[serde(rename = "exitBehavior")]
    pub exit_behavior: ExitBehavior,
    pub language: Language,
}

impl Default for SystemConfig {
    fn default() -> Self {
        let sys_locale = get_sys_locale();
        SystemConfig {
            theme: Theme::Light,
            theme_option: ThemeOption::User,
            auto_start: false,
            exit_behavior: ExitBehavior::Exit,
            language: if sys_locale == "zh-CN" {
                Language::ZhCN
            } else {
                Language::EnUS
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeOption {
    User,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExitBehavior {
    Minimize,
    Exit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    ZhCN,
    EnUS,
}
