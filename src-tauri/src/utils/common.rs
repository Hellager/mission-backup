#[allow(dead_code)]
/// Get the program name.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use common::get_program_name;
/// 
/// println!("Program name: {}", get_program_name());
/// ```
pub fn get_program_name() -> String {
    const NAME: Option<&str> = option_env!("CARGO_PKG_NAME");

    return NAME.unwrap_or("").to_string();
}

#[allow(dead_code)]
/// Get the program version.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use common::get_program_version;
/// 
/// println!("Program version: {}", get_program_version());
/// ```
pub fn get_program_version() -> String {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

    return VERSION.unwrap_or("").to_string();
}

#[allow(dead_code)]
/// Get the program homepage.
/// 
/// If `homepage` is not defined in `cargo.toml`, it will return `""`.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use common::get_program_homepage;
/// 
/// println!("Program homepage: {}", get_program_homepage());
/// ```
pub fn get_program_homepage() -> String {
    const HOMEPAGE: Option<&str> = option_env!("CARGO_PKG_HOMEPAGE");

    return HOMEPAGE.unwrap_or("").to_string();
}

#[allow(dead_code)]
/// Get the program repository.
/// 
/// If `repository` is not defined in `cargo.toml`, it will return `""`.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use common::get_program_repository;
/// 
/// println!("Program repository: {}", get_program_repository());
/// ```
pub fn get_program_repository() -> String {
    const REPOSITORY: Option<&str> = option_env!("CARGO_PKG_REPOSITORY");

    return REPOSITORY.unwrap_or("").to_string();
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
/// Gets system locale.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use common::get_sys_locale;
/// 
/// let locale = get_sys_locale();
/// println!("current locale: {:?}", locale),
/// ```
pub fn get_sys_locale() -> String {
    use sys_locale::get_locale;

    get_locale().unwrap_or_else(|| String::from("en-US"))
}

#[allow(dead_code)]
/// Get system theme.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use common::get_sys_theme;
/// 
/// let theme = get_sys_theme();
/// println!("current theme: {:?}", theme),
/// ```
pub fn get_sys_theme() -> String {
    let mode = dark_light::detect();

    if mode == dark_light::Mode::Dark { "dark".to_string() } else { "light".to_string() }
}

#[allow(dead_code)]
/// Get system webview versoin.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use common::get_webview_version;
/// 
/// let res = get_webview_version();
/// 
/// match res {
///     Ok(version) => println!("webview version: {:?}", version),
///     Err(error) => println!("get webview version failed, errMsg: {:?}", error),
/// }
/// ```
pub fn get_webview_version() -> Result<String, wry::Error> {
    use wry::webview::webview_version;

    webview_version()
}

#[allow(dead_code)]
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

#[allow(dead_code)]
/// Generate a random i32 number
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use common::rand_number;
/// 
/// let number = rand_number();
/// println!("the random number is {}", number);
/// ```
pub fn rand_number() -> i32 {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    rng.gen::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_program_metadata() {
        assert_eq!(get_program_name(), env!("CARGO_PKG_NAME"));
        assert_eq!(get_program_version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(get_program_homepage(), env!("CARGO_PKG_HOMEPAGE"));
        assert_eq!(get_program_repository(), env!("CARGO_PKG_REPOSITORY"));
    }

    #[test]
    #[ignore]
    fn test_exit_app() {
        exit_app();
    }

    #[test]
    fn test_get_app_home_dir() {
        use std::env::current_dir;

        let std_dir = current_dir().expect("");
        let home_dir = get_app_home_dir().expect("");

        assert_ne!(std_dir.display().to_string(), "".to_string());
        assert_ne!(home_dir.display().to_string(), "".to_string());
        assert_eq!(std_dir, home_dir);
    }

    #[test]
    fn test_get_timestamp() {
        let std_timestamp = get_current_timestamp_with_std().unwrap_or(0);
        let chrono_timestamp = get_current_timestamp().unwrap_or(0);

        assert_ne!(std_timestamp, 0);
        assert_ne!(chrono_timestamp, 0);
        assert_eq!(std_timestamp, chrono_timestamp as u64);
    }

    #[test]
    fn test_get_sys_locale() {
        let sys_locale = sys_locale::get_locale().expect("en-US");
        let locale = get_sys_locale();

        assert_ne!(locale, "".to_string());
        assert_eq!(locale, sys_locale);
    }

    #[test]
    fn test_get_sys_theme() {
        let sys_theme = dark_light::detect();
        let theme = get_sys_theme();

        assert_ne!(theme, "".to_string());
        match sys_theme {
            dark_light::Mode::Dark => {
                assert_eq!(theme, "dark");
            },
            dark_light::Mode::Light => {
                assert_eq!(theme, "light");
            },
            dark_light::Mode::Default => {
                assert_eq!(theme, "light");
            }
        }
    }

    #[test]
    fn test_get_webview_version() {
        let sys_webview_version = wry::webview::webview_version();
        let version = get_webview_version().expect("");

        if let Ok(webview_version) = sys_webview_version {
            assert_eq!(webview_version, version);
        } else {
            assert_eq!(version, "");
        }
    }

    #[test]
    #[ignore]
    fn test_open_url() {
        let url = "https://www.google.com";

        open_url_with_browser(url).expect("failed to open url with browser");
    }
}