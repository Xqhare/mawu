use std::fmt;

#[derive(Debug)]
/// `TomlError` wraps all errors the TOML side of Mawu can throw
pub enum TomlError {
    /// A wrapper for all TOML parsing errors
    ParseError(TomlParseError),
    /// A wrapper for all TOML writing errors
    WriteError(TomlWriteError),
}

pub type Result<T> = std::result::Result<T, TomlError>;

impl fmt::Display for TomlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TomlError::ParseError(ref e) => e.fmt(f),
            TomlError::WriteError(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for TomlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            TomlError::ParseError(ref e) => Some(e),
            TomlError::WriteError(ref e) => Some(e),
        }
    }
}

#[derive(Debug)]
/// `CsvWriteError` wraps all writing errors
pub enum TomlWriteError {
    /// Supplied value is not a TOML value
    NotTOML,
    /// Supplied value is not a TOML value
    NotTOMLType(String),
    /// Supplied Value is not an Object
    ParentMustBeObject,
}

impl fmt::Display for TomlWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TomlWriteError::ParentMustBeObject => write!(
                f,
                "Parent (`XffValue` passed into the function) must be an object"
            ),
            TomlWriteError::NotTOML => write!(f, "Supplied value is not a TOML value"),
            TomlWriteError::NotTOMLType(ref s) => write!(f, "Not TOML type: {s}"),
        }
    }
}

impl std::error::Error for TomlWriteError {}

#[derive(Debug)]
/// `TomlParseError` wraps all parsing errors
pub enum TomlParseError {
    /// Encountered an unescaped double quote
    UnescapedDoubleQuote,
    /// Encountered an unterminated quote
    UnterminatedQuote,
    /// Encountered an unescaped character that should be
    UnescapedCharacter(char),
    /// Encountered an unexpected newline
    UnexpectedNewline,
    /// Encountered unexpected end of file
    UnexpectedEndOfFile,
    /// Encountered an unexpected character
    UnexpectedCharacter(String),
    /// Encountered an invalid structural token
    InvalidStructuralToken(String),
    /// Encountered an invalid character
    InvalidCharacter(String),
    /// Encountered an invalid escape sequence
    InvalidEscapeSequence(String),
    /// Expected colon, got something else
    ExpectedColon,
    /// Key already defined, Value cannot hold another key value pair or single value
    KeyAlreadyDefined,
    /// Expected key, got something else
    ExpectedKey,
    /// Expected value, got something else
    ExpectedValue,
    /// Expected end of object, got something else
    ExpectedEndOfObject,
    /// Encountered `NaN` or `Infinity`
    InvalidNumber(String),
}

impl fmt::Display for TomlParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TomlParseError::KeyAlreadyDefined => write!(f, "Key already defined"),
            TomlParseError::UnescapedDoubleQuote => write!(f, "Unescaped double quote"),
            TomlParseError::UnterminatedQuote => write!(f, "Unterminated quote"),
            TomlParseError::UnescapedCharacter(c) => write!(f, "Unescaped character: {c}"),
            TomlParseError::UnexpectedNewline => write!(f, "Unexpected newline"),
            TomlParseError::InvalidStructuralToken(ref s) => {
                write!(f, "Invalid structural token: {s}")
            }
            TomlParseError::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),
            TomlParseError::InvalidCharacter(ref s) => write!(f, "Invalid character: {s}"),
            TomlParseError::InvalidEscapeSequence(ref s) => {
                write!(f, "Invalid escape sequence: {s}")
            }
            TomlParseError::ExpectedColon => write!(f, "Expected colon"),
            TomlParseError::ExpectedKey => write!(f, "Expected key"),
            TomlParseError::ExpectedValue => write!(f, "Expected value"),
            TomlParseError::UnexpectedCharacter(ref s) => write!(f, "Unexpected character: {s}"),
            TomlParseError::ExpectedEndOfObject => write!(f, "Expected end of object"),
            TomlParseError::InvalidNumber(ref s) => write!(f, "Invalid number: {s}"),
        }
    }
}

impl std::error::Error for TomlParseError {}
