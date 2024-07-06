use std::fmt;

#[derive(Debug)]
/// JsonError wraps all errors the JSON side of Mawu can throw
pub enum JsonError {
    /// A wrapper for all JSON parsing errors
    ParseError(JsonParseError),
}

pub type Result<T> = std::result::Result<T, JsonError>;

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsonError::ParseError(ref e) => e.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum JsonParseError {
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
    /// Expected key, got something else
    ExpectedKey,
    /// Expected value, got something else
    ExpectedValue,
    /// Expected end of object, got something else
    ExpectedEndOfObject,
}

impl fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsonParseError::UnescapedDoubleQuote => write!(f, "Unescaped double quote"),
            JsonParseError::UnterminatedQuote => write!(f, "Unterminated quote"),
            JsonParseError::UnescapedCharacter(c) => write!(f, "Unescaped character: {}", c),
            JsonParseError::UnexpectedNewline => write!(f, "Unexpected newline"),
            JsonParseError::InvalidStructuralToken(ref s) => {
                write!(f, "Invalid structural token: {}", s)
            }
            JsonParseError::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),
            JsonParseError::InvalidCharacter(ref s) => write!(f, "Invalid character: {}", s),
            JsonParseError::InvalidEscapeSequence(ref s) => {
                write!(f, "Invalid escape sequence: {}", s)
            }
            JsonParseError::ExpectedColon => write!(f, "Expected colon"),
            JsonParseError::ExpectedKey => write!(f, "Expected key"),
            JsonParseError::ExpectedValue => write!(f, "Expected value"),
            JsonParseError::UnexpectedCharacter(ref s) => write!(f, "Unexpected character: {}", s),
            JsonParseError::ExpectedEndOfObject => write!(f, "Expected end of object"),
        }
    }
}
