//! # Mawu
//! A JSON and CSV serialization and deserialization library written in rust.
//! 
//! Mawu, named after the ancient creator goddess Mawu in West African mythology, offers a JSON and CSV serialization and deserialization library implementing the rfc4180, rfc8259 and the ECMA-404 standard.
//!
//! Mawu is a zero dependency library and supports 64bit systems only.
//!
//! ***This is a hobbyist repo badly reinventing the wheel and not ready for production use.*** 
//!
//! ## Features
//! - Simple
//! - Type aware
//! - Supports both CSV and JSON
//! - Reading and writing
//! - Write pretty with custom spacing
//! - Supports CSV files with or without header
//! - Supports missing or not provided values
//! - Fully documented
//! - Tries to stay as close to the rfc4180, rfc8259 and ECMA-404 standard as possible for maximum interoperability
//! - Actually written by a human
//!
//! ## Using Mawu
//!
//! ```rust
//! use mawu::*;
//! use athena::XffValue;
//!
//! // most of the time you will ever only need
//! use mawu::read::json;
//! // or one of these two
//! use mawu::read::{csv_headed, csv_headless};
//!
//! // JSON returns an XffValue directly
//! # let path_to_file = "data/json/json-test-data/simple-object.json";
//! # if std::path::Path::new(path_to_file).exists() {
//! let xff_value = json(path_to_file).unwrap();
//! if xff_value.is_object() {
//!     for (key, value) in xff_value.into_object().unwrap().iter() {
//!         println!("{}: {}", key, value);
//!     }
//! }
//! # }
//!
//! // CSV returns a MawuValue
//! # let path_to_csv = "data/csv/csv-test-data/headed/my-own-random-data/all-types.csv";
//! # if std::path::Path::new(path_to_csv).exists() {
//! let csv_value = csv_headed(path_to_csv).unwrap();
//! if csv_value.is_csv_object() {
//!     for row in csv_value.as_csv_object().unwrap() {
//!         for (key, value) in row {
//!             println!("{}: {}", key, value);
//!         }
//!     }
//! }
//! # }
//!
//! // to save to a file use write or write_pretty
//! use mawu::{write, write_pretty, MawuContents};
//!
//! let path_to_file = "example-file.json";
//! let xff_val = XffValue::from(vec![1, 2, 3]);
//! // Use MawuContents::Json for XffValue
//! write(path_to_file, MawuContents::Json(xff_val)).unwrap();
//! # std::fs::remove_file(path_to_file).unwrap();
//! ```
//!
//! ## `MawuValue`
//! In the new version of Mawu, `MawuValue` is used exclusively for CSV data.
//! It serves as a container for either a headed CSV (`CSVObject`) or a headless CSV (`CSVArray`),
//! wrapping `athena::XffValue` for the individual fields.
//!
//! For JSON data, Mawu now returns `athena::XffValue` directly, providing a more direct and 
//! standardized way to interact with JSON structures.
//!
//! ## `MawuContents`
//! To maintain a unified writing API, the `MawuContents` enum is used to wrap either 
//! `XffValue` (for JSON) or `MawuValue` (for CSV) when calling `write` or `write_pretty`.
//!

/// Contains all the errors that can be returned by Mawu
pub mod errors;
/// Contains a wrapper for all data values supported by Mawu
pub mod mawu_value;
/// Contains all the lexers for CSV and JSON files
mod lexers;
/// Contains all the serializers for CSV and JSON files
mod serializers;
/// Contains all utility functions
mod utils;

/// Reads CSV and JSON files into `MawuValue` or `XffValue`
pub mod read {
    use std::path::Path;
    use athena::XffValue;

    use crate::{
        errors::MawuError,
        lexers::{csv_lexer, json_lexer},
        mawu_value::MawuValue,
        utils::file_handling,
    };

    /// Reads a headed CSV file and returns a `MawuValue::CSVObject` or an error if the file could not be read or parsed.
    ///
    /// Call `as_csv_object` or `to_csv_object` on the result to get the `Vec<HashMap<String, XffValue>>`
    ///
    /// # Arguments
    /// * `path` - The path to the CSV file, relative or absolute
    ///
    /// # Errors
    /// Only returns `MawuError`'s
    pub fn csv_headed<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        csv_lexer::headed(
            file_handling::read_file(path)?
        )
    }

    /// Reads a headless CSV file and returns a `MawuValue::CSVArray` or an error if the file could not be read or parsed.
    ///
    /// Call `as_csv_array` or `to_csv_array` on the result to get the `Vec<Vec<XffValue>>`
    ///
    /// # Arguments
    /// * `path` - The path to the CSV file, relative or absolute
    ///
    /// # Errors
    /// Only returns `MawuError`'s
    pub fn csv_headless<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        csv_lexer::headless(
            file_handling::read_file(path)?
        )
    }

    /// Reads a JSON file and returns a `XffValue` or an error if the file could not be read or parsed.
    ///
    /// # Arguments
    /// * `path` - The path to the JSON file, relative or absolute
    ///
    /// # Errors
    /// Only returns `MawuError`'s
    pub fn json<T: AsRef<Path>>(path: T) -> Result<XffValue, MawuError> {
        json_lexer::json_lexer(
            file_handling::read_file(path)?
        )
    }
}

use std::path::Path;
use athena::XffValue;
use crate::{errors::MawuError, mawu_value::MawuValue, serializers::{csv_serializer, json_serializer}, utils::file_handling::write_file};

/// Enum to unify JSON and CSV data for writing
pub enum MawuContents {
    /// JSON data represented by `XffValue`
    Json(XffValue),
    /// CSV data represented by `MawuValue`
    Csv(MawuValue),
}

impl From<XffValue> for MawuContents {
    fn from(v: XffValue) -> Self {
        MawuContents::Json(v)
    }
}

impl From<MawuValue> for MawuContents {
    fn from(v: MawuValue) -> Self {
        MawuContents::Csv(v)
    }
}

/// Writes a file with the given contents.
/// Writes a CSV-file if the contents are `MawuContents::Csv` and a JSON-file if the contents are `MawuContents::Json`.
///
/// ## Arguments
/// * `path` - The path to the file, relative or absolute
/// * `contents` - The contents of the file
pub fn write<T: AsRef<Path>, C: Into<MawuContents>>(path: T, contents: C) -> Result<(), MawuError> {
    write_pretty(path, contents, 0)
}

/// Writes a pretty printed file with the given contents.
/// Writes a CSV-file if the contents are `MawuContents::Csv` and a JSON-file if the contents are `MawuContents::Json`.
///
/// ## Arguments
/// * `path` - The path to the file, relative or absolute
/// * `contents` - The contents of the file
/// * `space` - The number of spaces to use for indentation
pub fn write_pretty<T: AsRef<Path>, C: Into<MawuContents>>(path: T, contents: C, spaces: u8) -> Result<(), MawuError> {
    let contents = contents.into();
    match contents {
        MawuContents::Csv(MawuValue::CSVObject(v)) => write_file(path, csv_serializer::serialize_csv_headed(MawuValue::CSVObject(v), spaces)?),
        MawuContents::Csv(MawuValue::CSVArray(v)) => write_file(path, csv_serializer::serialize_csv_unheaded(MawuValue::CSVArray(v), spaces)?),
        MawuContents::Json(v) => write_file(path, json_serializer::serialize_json(v, spaces, 0)?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn write_json_doc_files() {
        let path_to_file1 = "json_output_pretty.json";
        let path_to_file2 = "json_output.json";

        let mut object = athena::Object::new();
        object.insert("key1".to_string(), XffValue::from("value1"));
        object.insert("key2".to_string(), XffValue::from(2));
        let json_value1 = XffValue::Object(object);

        write_pretty(path_to_file1, MawuContents::Json(json_value1), 4).expect("Failed to write JSON file");

        let json_value2 = XffValue::from(vec![
            XffValue::from("a"),
            XffValue::from(1),
            XffValue::from(vec![
                XffValue::from(-1),
                XffValue::from(true),
            ]),
        ]);
        write(path_to_file2, MawuContents::Json(json_value2)).expect("Failed to write JSON file");

        let read_json1 = read::json(path_to_file1).unwrap();
        let read_json2 = read::json(path_to_file2).unwrap();

        assert!(read_json1.is_object());
        assert_eq!(read_json1.into_object().unwrap().get("key1").unwrap().into_string().unwrap(), "value1");
        assert_eq!(read_json1.into_object().unwrap().get("key2").unwrap().into_number().unwrap().into_usize().unwrap(), 2);

        assert!(read_json2.is_array());
        assert_eq!(read_json2.len(), 3);
        assert_eq!(read_json2.into_array().unwrap().get(0).unwrap().into_string().unwrap(), "a");

        std::fs::remove_file(path_to_file1).unwrap();
        std::fs::remove_file(path_to_file2).unwrap();
    }

    #[test]
    fn write_csv() {
        let path_to_file = "csv_output_pretty2.csv";

        let mut row0 = HashMap::new();
        row0.insert("key1".to_string(), XffValue::from("value1"));
        row0.insert("key2".to_string(), XffValue::from(2));

        let mut row1 = HashMap::new();
        row1.insert("key1".to_string(), XffValue::from("value2"));
        row1.insert("key2".to_string(), XffValue::from(3));

        let csv_value = MawuValue::CSVObject(vec![row0, row1]);

        write_pretty(path_to_file, MawuContents::Csv(csv_value), 4).unwrap();
        let read_csv = read::csv_headed(path_to_file).unwrap();
        assert!(read_csv.is_csv_object());
        assert_eq!(read_csv.as_csv_object().unwrap()[0].get("key1").unwrap().into_string().unwrap(), "value1");

        std::fs::remove_file(path_to_file).unwrap();
    }
}
