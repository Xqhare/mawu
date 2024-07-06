use std::fmt;

#[derive(Debug)]
/// CsvError wraps all errors the CSV side of Mawu can throw
pub enum CsvError {
    /// A wrapper for all parsing errors
    ParseError(CsvParseError),
}

pub type Result<T> = std::result::Result<T, CsvError>;

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CsvError::ParseError(ref e) => e.fmt(f),
        }
    }
}

#[derive(Debug)]
/// CsvParseError wraps all parsing errors
pub enum CsvParseError {
    /// Encountered an unescaped double quote
    UnescapedDoubleQuote,
    /// Encountered an unterminated quote
    UnterminatedQuote,
    /// Encountered an unescaped character that should not be
    UnescapedCharacter(char),
    /// Encountered an extra value
    ExtraValue(String),
    /// Encountered an unrecognized header
    UnrecognizedHeader(String),
    /// Encountered an unexpected newline
    UnexpectedNewline,
}

impl fmt::Display for CsvParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CsvParseError::UnescapedDoubleQuote => write!(f, "Unescaped double quote"),
            CsvParseError::UnterminatedQuote => write!(f, "Unterminated quote"),
            CsvParseError::UnescapedCharacter(c) => write!(f, "Unescaped character: {}", c),
            CsvParseError::ExtraValue(ref s) => write!(f, "Extra value: {}", s),
            CsvParseError::UnexpectedNewline => write!(f, "Unexpected newline"),
            CsvParseError::UnrecognizedHeader(ref s) => write!(f, "Unrecognized header: {}", s),
        }
    }
}
