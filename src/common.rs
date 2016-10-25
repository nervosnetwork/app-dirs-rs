use std;

/// Struct that holds information about your app.
/// 
/// It's recommended to create a single `const` instance of `AppInfo`:
///
/// ```
/// use app_dirs::AppInfo;
/// const APP_INFO: AppInfo = AppInfo{name: "Awesome App", author: "Dedicated Dev"};
/// ```
///
/// # Caveats
/// Functions in this library sanitize any characters that could be
/// non-filename-safe from `name` and `author`. The resulting paths will be
/// more human-readable if you stick to **letters, numbers, spaces, hyphens,
/// underscores, and periods** for both properties.
///
/// The `author` property is currently only used by Windows, as macOS and *nix
/// specifications don't require it. Make sure your `name` string is unique!
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AppInfo {
    /// Name of your app (e.g. "Hearthstone").
    pub name: &'static str,
    /// Author of your app (e.g. "Blizzard").
    pub author: &'static str,
}

/// Enum specifying the type of app data you want to store.
///
/// **Different platforms are NOT guaranteed to distinguish between each data
/// type.** Keep this in mind when choosing data file paths.
///
/// Example: Windows does not supported shared application data and does not
/// distinguish between config and data. Therefore, on Windows, all variants
/// except `UserCache` return the same path.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppDataType {
    /// User-specific app configuration data.
    UserConfig,
    /// User-specific arbitrary app data.
    UserData,
    /// User-specific app cache data.
    UserCache,
    /// System-wide arbitrary app data.
    SharedData,
    /// System-wide app configuration data.
    SharedConfig,
}

impl AppDataType {
    /// Returns `true` for non-user-specific data types.
    pub fn is_shared(&self) -> bool {
        use AppDataType::*;
        match *self {
            SharedData | SharedConfig => true,
            _ => false,
        }
    }
}

/// Error type for any `app_dirs` operation.
#[derive(Debug)]
pub enum AppDirsError {
    /// An I/O error occurred during the operation.
    Io(std::io::Error),
    /// App-specific directories are not properly supported by the system
    /// (e.g. required environment variables don't exist).
    NotSupported,
    /// App info given to this library was invalid (e.g. app name or author
    /// were empty).
    InvalidAppData,
}

impl From<std::io::Error> for AppDirsError {
    fn from(e: std::io::Error) -> Self {
        AppDirsError::Io(e)
    }
}
