use crate::config::ConfigError;
#[cfg(feature = "renderer")]
use crate::sugarloaf::font::SugarloafFont;

#[derive(Clone, Copy, PartialEq)]
pub enum MarioErrorLevel {
    Warning,
    Error,
}

#[derive(Clone)]
pub struct MarioError {
    pub report: MarioErrorType,
    pub level: MarioErrorLevel,
}

impl MarioError {
    pub fn configuration_not_found() -> Self {
        MarioError {
            level: MarioErrorLevel::Warning,
            report: MarioErrorType::ConfigurationNotFound,
        }
    }
}

impl From<ConfigError> for MarioError {
    fn from(error: ConfigError) -> Self {
        match error {
            ConfigError::ErrLoadingConfig(message) => MarioError {
                report: MarioErrorType::InvalidConfigurationFormat(message),
                level: MarioErrorLevel::Warning,
            },
            ConfigError::ErrLoadingTheme(message) => MarioError {
                report: MarioErrorType::InvalidConfigurationTheme(message),
                level: MarioErrorLevel::Warning,
            },
            ConfigError::PathNotFound => MarioError {
                report: MarioErrorType::ConfigurationNotFound,
                level: MarioErrorLevel::Warning,
            },
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum MarioErrorType {
    // font was not found
    #[cfg(feature = "renderer")]
    FontsNotFound(Vec<SugarloafFont>),

    // navigation configuration has changed
    // NavigationHasChanged,
    InitializationError(String),

    // configurlation file was not found
    ConfigurationNotFound,
    // configuration file have an invalid format
    InvalidConfigurationFormat(String),
    // configuration invalid theme
    InvalidConfigurationTheme(String),

    // background image referenced in config could not be loaded
    BackgroundImageLoadFailure(String),

    // reports that are ignored by MarioErrorType
    IgnoredReport,
}

impl std::fmt::Display for MarioErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            #[cfg(feature = "renderer")]
            MarioErrorType::FontsNotFound(fonts) => {
                let mut font_str = String::from("");
                for font in fonts.iter() {
                    font_str += format!("\n• \"{}\"", font.family).as_str();
                }

                write!(f, "Font(s) not found:\n{font_str}")
            }
            MarioErrorType::ConfigurationNotFound => {
                write!(f, "Configuration file was not found")
            }
            // MarioErrorType::NavigationHasChanged => {
            //     write!(f, "Navigation has changed\n\nPlease reopen Mario terminal.")
            // }
            MarioErrorType::InitializationError(message) => {
                write!(f, "Error initializing Mario terminal:\n{message}")
            }
            MarioErrorType::IgnoredReport => write!(f, ""),
            MarioErrorType::InvalidConfigurationFormat(message) => {
                write!(f, "Found an issue loading the configuration file:\n\n{message}\n\nMario will proceed with the default configuration")
            }
            MarioErrorType::InvalidConfigurationTheme(message) => {
                write!(f, "Found an issue in the configured theme:\n\n{message}")
            }
            MarioErrorType::BackgroundImageLoadFailure(message) => {
                write!(
                    f,
                    "Could not load the configured background image:\n\n{message}\n\nCheck `window.background-image.path` in your config."
                )
            }
        }
    }
}
