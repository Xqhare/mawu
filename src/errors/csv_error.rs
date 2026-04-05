use std::fmt;

#[derive(Debug)]
/// `CsvError` wraps all errors the CSV side of Mawu can throw
pub enum CsvError {
    /// A wrapper for all parsing errors
    ParseError(CsvParseError),
    /// A wrapper for all writing errors
    WriteError(CsvWriteError),
}

pub type Result<T> = std::result::Result<T, CsvError>;

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CsvError::ParseError(ref e) => e.fmt(f),
            CsvError::WriteError(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for CsvError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            CsvError::ParseError(ref e) => Some(e),
            CsvError::WriteError(ref e) => Some(e),
        }
    }
}

#[derive(Debug)]
/// `CsvWriteError` wraps all writing errors
pub enum CsvWriteError {
    /// Supplied value is not a CSV value
    NotCSV,
    /// Unallowed `MawuValue`
    UnallowedType(String),
}

impl fmt::Display for CsvWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CsvWriteError::NotCSV => write!(f, "Supplied value is not a CSV value"),
            CsvWriteError::UnallowedType(ref s) => write!(f, "Unallowed type: {s}"),
        }
    }
}

impl std::error::Error for CsvWriteError {}

#[derive(Debug)]
/// `CsvParseError` wraps all parsing errors
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
            CsvParseError::UnescapedCharacter(c) => write!(f, "Unescaped character: {c}"),
            CsvParseError::ExtraValue(ref s) => write!(f, "Extra value: {s}"),
            CsvParseError::UnexpectedNewline => write!(f, "Unexpected newline"),
            CsvParseError::UnrecognizedHeader(ref s) => write!(f, "Unrecognized header: {s}"),
        }
    }
}

impl std::error::Error for CsvParseError {}
