//! # Cmd
//! 
//! `Cmd` module contains all commands that interacts with frontend.

use tauri::{Manager, Window};
use serde::{Serialize, Deserialize};
use log::error;

/// Struct for command response
#[derive(Clone, Serialize, Deserialize)]
pub struct Response<T> {
    /// Response status code
    /// More like HTTP response status codes
    pub code: i32,

    /// Response data
    /// Should be able to be serialize and deserialize
    pub data: T,

    /// Additional message
    pub msg: String,
}

impl<T> Response<T> {
    pub fn success(value: T) -> Response<T> {
        Response {
            code: 200,
            data: value,
            msg: "".to_string(),
        }
    }

    pub fn error(err_code: i32, err_msg: String) -> Response<bool> {
        Response {
            code: err_code,
            data: false,
            msg: err_msg
        }
    }
}

#[tauri::command]
pub fn show_main_window(window: Window) -> Result<Response<bool>, Response<bool>> {
    if let Some(splashscreen) = window.get_window("splashscreen") {
        if let Err(error) = splashscreen.close() {
            error!("failed to init app, errMsg: {:?}", error);
            return Err(Response::<bool>::error(500, format!("{:?}", error)));
        }
    } else {
        error!("missing splashsceen window");
        return Err(Response::<bool>::error(404, format!("missing splashsceen window")));
    }

    // Show main window
    if let Some(main_window) = window.get_window("main") {
        if let Err(error) = main_window.show() {
            error!("failed to show main window, errMsg: {:?}", error);
            return Err(Response::<bool>::error(500, format!("{:?}", error)));
        }
    } else {
        error!("missing main window");
        return Err(Response::<bool>::error(404, format!("missing main window")));
    }

    Ok(Response::success(true))
}
