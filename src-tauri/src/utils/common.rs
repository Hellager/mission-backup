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
    use std::env::current_dir;
    
    current_dir()
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
    if let Ok(mode) = dark_light::detect() {
        if mode == dark_light::Mode::Dark {
            "dark".to_string()
        } else {
            "light".to_string()
        }
    } else {
        "light".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_app_home_dir() {
        let result = get_app_home_dir();
        assert!(result.is_ok(), "Should successfully get app directory");
        let path = result.unwrap();
        assert!(path.exists(), "Directory should exist");
        assert!(path.is_absolute(), "Should return absolute path");
    }

    #[test]
    fn test_get_sys_locale() {
        let locale = get_sys_locale();
        assert!(!locale.is_empty(), "Locale should not be empty");
        assert!(locale.contains('-'), "Locale should contain language and region codes");
        
        // Verify format matches xx-XX pattern
        let parts: Vec<&str> = locale.split('-').collect();
        assert_eq!(parts.len(), 2, "Locale should contain two parts");
    }

    #[test]
    fn test_get_sys_theme() {
        let theme = get_sys_theme();
        assert!(
            theme == "dark" || theme == "light",
            "Theme should be either 'dark' or 'light'"
        );
    }
}
