use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to get config file path")]
    ConfigPathUnavailable,

    #[error("Failed to convert config to/from TOML: {0}")]
    TomlConvertFailed(String),

    #[error("Failed to parse TOML config: {0}")]
    TomlParseFailed(String),

    #[error("I/O error: {0}")]
    IoError(String),

    #[error("Failed to save config: {0}")]
    ConfigSaveFailed(String),

    #[error("Window operation error: {0}")]
    WindowError(String),

    #[error("{0}")]
    Other(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(err: toml::ser::Error) -> Self {
        AppError::TomlConvertFailed(err.to_string())
    }
}

impl From<toml::de::Error> for AppError {
    fn from(err: toml::de::Error) -> Self {
        AppError::TomlParseFailed(err.to_string())
    }
}

impl From<tauri::Error> for AppError {
    fn from(err: tauri::Error) -> Self {
        AppError::WindowError(err.to_string())
    }
}

impl From<tauri_plugin_window_state::Error> for AppError {
    fn from(err: tauri_plugin_window_state::Error) -> Self {
        AppError::WindowError(err.to_string())
    }
}

