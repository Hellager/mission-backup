mod common {
    //! The `common` module contains commonly used function.
    //! 
    //! # Install Dependencies
    //! 
    //! ```
    //! $ cargo add chrono sys_locale
    //! ```

    /// Exit app.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use common::exit_app;
    /// 
    /// let _ = exit_app();
    /// ```
    pub fn exit_app() {
        use std::process;

        process::exit(0)
    }

    /// Get app home directory, not the exe directory
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use common::get_app_home_dir;
    /// 
    /// let app_dir = get_app_home_dir();
    /// match app_dir {
    ///     Ok(dir) => println!("app running at: {:?}", dir),
    ///     Err(error) => println!("get app home dir failed, errMsg: {:?}", error),
    /// }
    /// ```
    pub fn get_app_home_dir() -> Result<std::path::PathBuf, std::io::Error> {
        #[cfg(target_os = "windows")]
        return std::env::current_dir();

        #[cfg(not(target_os = "windows"))]
        match std::env::home_dir() {
            None => {
                Err(ErrorKind::NotFound)  
            },
            Some(path) => {
                Ok(path.join(APP_DIR))
            }
        }
    }    

    /// Gets current timestamp with standard library.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use common::get_current_timestamp_with_std;
    /// 
    /// let res = get_current_timestamp_with_std();
    /// match res {
    ///     Ok(timestamp) => println!("current timestamp: {:?}", timestamp),
    ///     Err(error) => println!("get current timestamp failed, errMsg: {:?}", error),
    /// }
    /// ```
    pub fn get_current_timestamp_with_std() -> Result<u64, std::time::SystemTimeError> {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now();
        let timestamp = now.duration_since(UNIX_EPOCH)?.as_secs();

        Ok(timestamp)
    }

    /// Gets current timestamp.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use common::get_current_timestamp;
    /// 
    /// let res = get_current_timestamp();
    /// match res {
    ///     Ok(timestamp) => println!("current timestamp: {:?}", timestamp),
    ///     Err(error) => println!("get current timestamp failed, errMsg: {:?}", error),
    /// }
    /// ```
    pub fn get_current_timestamp() -> Result<i64, std::time::SystemTimeError> {
        use chrono::Utc;

        Ok(Utc::now().timestamp())
    }

    /// Gets system locale.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use common::get_sys_locale;
    /// 
    /// let res = get_sys_locale();
    /// match res {
    ///     Ok(locale) => println!("current locale: {:?}", locale),
    ///     Err(error) => println!("get current locale failed, errMsg: {:?}", error),
    /// }
    /// ```
    pub fn get_sys_locale() -> Result<String, std::io::Error> {
        use sys_locale::get_locale;

        Ok(get_locale().unwrap_or_else(|| String::from("en-US")))
    }

    /// Opens url with default browser.
    /// 
    /// # Arguments
    /// 
    /// * `url` - A string that holds the url
    /// 
    /// # Examples
    /// 
    /// ```
    /// use common::open_url_with_browser;
    /// 
    /// let _ = open_url_with_browser("https://www.google.com").unwrap();
    /// ```
    pub fn open_url_with_browser(url: &str) -> Result<(), std::io::Error> {
        use std::env::consts;
        use std::process::Command;
        use std::thread::sleep;
        use std::time::Duration;
        use std::io::{Error, ErrorKind};

        let os_command = match consts::OS {
            "linux" => "xdg_open",
            "macos" => "open",
            "windows" => "open",
            _ => "open"
        };

        if let Ok(mut child) = Command::new(os_command).arg(url).spawn() {
            sleep(Duration::new(3, 0));
            if let Ok(_status) = child.wait() {
                return Ok(());
            }
         }
        
        Err(Error::from(ErrorKind::Other))
    }
}
