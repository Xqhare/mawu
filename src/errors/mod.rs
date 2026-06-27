use std::result;

/// Module holding all possible csv errors
#[cfg(feature = "csv")]
pub mod csv_error;
/// Module holding all possible json errors
pub mod json_error;
/// Module holding all possible toml errors
pub mod toml_error;

pub type Result<T> = result::Result<T, nemesis::NemesisError>;

#[derive(Debug)]
/// Internal errors, If you ever see this, please file an issue.
pub enum MawuInternalError {
    /// Fail-safe if unable to lock the master mutex of the character queue
    UnableToLockMasterMutex,
    /// Fail-safe if Mawu encountered a String with no chars
    StringWithNoChars(String),
    /// Fail-save if unable to unescape unicode
    UnableToUnescapeUnicode(String),
    /// Fail-safe if Mawu encountered a String with no chars
    NotUTF8(String),
}

impl std::fmt::Display for MawuInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MawuInternalError::UnableToLockMasterMutex => write!(f, "Unable to lock mutex"),
            MawuInternalError::StringWithNoChars(ref s) => write!(f, "String with no chars: {s}"),
            MawuInternalError::UnableToUnescapeUnicode(ref s) => {
                write!(f, "Unable to unescape unicode: {s}")
            }
            MawuInternalError::NotUTF8(ref s) => write!(f, "Not UTF8: {s}"),
        }
    }
}

impl std::error::Error for MawuInternalError {}
