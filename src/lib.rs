#![doc = include_str!("../README.md")]

/// Contains all the errors that can be returned by Mawu
pub mod errors;
/// Contains all the lexers for CSV and JSON files
mod lexers;
/// Contains a wrapper for all data values supported by Mawu
pub mod mawu_value;
/// Contains all the serializers for CSV and JSON files
mod serializers;
/// Contains all utility functions
mod utils;

pub use athena::XffValue;
pub use mawu_value::MawuValue;

/// Reads CSV and JSON files into `MawuValue` or `XffValue`
pub mod read {
    use athena::XffValue;
    use std::path::Path;

    #[cfg(feature = "csv")]
    use crate::{lexers::csv_lexer, mawu_value::MawuValue};
    use crate::{
        lexers::{json_lexer, toml_lexer},
        utils::file_handling,
    };
    use nemesis::NemesisError;

    /// Reads a headed CSV file and returns a `MawuValue::CSVObject` or an error if the file could not be read or parsed.
    ///
    /// Call `as_csv_object` or `to_csv_object` on the result to get the `Vec<HashMap<String, XffValue>>`
    ///
    /// # Arguments
    /// * `path` - The path to the CSV file, relative or absolute
    ///
    /// # Errors
    /// Only returns `NemesisError`'s
    #[cfg(feature = "csv")]
    pub fn csv_headed<T: AsRef<Path>>(path: T) -> Result<MawuValue, NemesisError> {
        csv_lexer::headed(file_handling::read_file(path)?)
    }

    /// Reads a headless CSV file and returns a `MawuValue::CSVArray` or an error if the file could not be read or parsed.
    ///
    /// Call `as_csv_array` or `to_csv_array` on the result to get the `Vec<Vec<XffValue>>`
    ///
    /// # Arguments
    /// * `path` - The path to the CSV file, relative or absolute
    ///
    /// # Errors
    /// Only returns `NemesisError`'s
    #[cfg(feature = "csv")]
    pub fn csv_headless<T: AsRef<Path>>(path: T) -> Result<MawuValue, NemesisError> {
        csv_lexer::headless(file_handling::read_file(path)?)
    }

    /// Reads a JSON file and returns a `XffValue` or an error if the file could not be read or parsed.
    ///
    /// # Arguments
    /// * `path` - The path to the JSON file, relative or absolute
    ///
    /// # Errors
    /// Only returns `NemesisError`'s
    pub fn json<T: AsRef<Path>>(path: T) -> Result<XffValue, NemesisError> {
        json_lexer::json_lexer(file_handling::read_file(path)?)
    }

    /// Reads a TOML file and returns a `XffValue` or an error if the file could not be read or parsed.
    ///
    /// # Arguments
    /// * `path` - The path to the TOML file, relative or absolute
    ///
    /// # Errors
    /// Only returns `NemesisError`'s
    pub fn toml<T: AsRef<Path>>(path: T) -> Result<XffValue, NemesisError> {
        let mut line_number = 1;
        toml_lexer::toml_lexer(
            &file_handling::read_file_unicode_segment(path)?,
            &mut line_number,
        )
    }
}

#[cfg(feature = "csv")]
use crate::serializers::csv_serializer;
use crate::{
    serializers::json_serializer, utils::file_handling::write_file,
};
use nemesis::NemesisError;
use std::path::Path;

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
pub fn write<T: AsRef<Path>, C: Into<MawuContents>>(
    path: T,
    contents: C,
) -> Result<(), NemesisError> {
    write_pretty(path, contents, 0)
}

/// Writes a pretty printed file with the given contents.
/// Writes a CSV-file if the contents are `MawuContents::Csv` and a JSON-file if the contents are `MawuContents::Json`.
///
/// ## Arguments
/// * `path` - The path to the file, relative or absolute
/// * `contents` - The contents of the file
/// * `space` - The number of spaces to use for indentation
#[cfg(feature = "csv")]
pub fn write_pretty<T: AsRef<Path>, C: Into<MawuContents>>(
    path: T,
    contents: C,
    spaces: u8,
) -> Result<(), NemesisError> {
    let contents = contents.into();
    match contents {
        MawuContents::Csv(MawuValue::CSVObject(v)) => write_file(
            path,
            csv_serializer::serialize_csv_headed(MawuValue::CSVObject(v), spaces)?,
        ),
        MawuContents::Csv(MawuValue::Object(v)) => write_file(
            path,
            csv_serializer::serialize_csv_headed(MawuValue::Object(v), spaces)?,
        ),
        MawuContents::Csv(MawuValue::OrderedObject(v)) => write_file(
            path,
            csv_serializer::serialize_csv_headed(MawuValue::OrderedObject(v), spaces)?,
        ),
        MawuContents::Csv(MawuValue::Table(v)) => write_file(
            path,
            csv_serializer::serialize_csv_headed(MawuValue::Table(v), spaces)?,
        ),
        MawuContents::Csv(MawuValue::CSVArray(v)) => write_file(
            path,
            csv_serializer::serialize_csv_unheaded(MawuValue::CSVArray(v), spaces)?,
        ),
        MawuContents::Json(v) => write_file(path, json_serializer::serialize_json(v, spaces, 0)?),
    }
}

/// Writes a pretty printed file with the given contents.
/// Writes a CSV-file if the contents are `MawuContents::Csv` and a JSON-file if the contents are `MawuContents::Json`.
///
/// ## Arguments
/// * `path` - The path to the file, relative or absolute
/// * `contents` - The contents of the file
/// * `space` - The number of spaces to use for indentation
#[cfg(not(feature = "csv"))]
pub fn write_pretty<T: AsRef<Path>, C: Into<MawuContents>>(
    path: T,
    contents: C,
    spaces: u8,
) -> Result<(), NemesisError> {
    let contents = contents.into();
    match contents {
        MawuContents::Csv(_) => Err(NemesisError::new(
            "mawu::write_pretty",
            "CSV serialization not enabled; Enable the 'csv' feature.",
        )),
        MawuContents::Json(v) => write_file(path, json_serializer::serialize_json(v, spaces, 0)?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "csv")]
    use std::collections::HashMap;

    #[test]
    fn write_json_doc_files() {
        let path_to_file1 = "json_output_pretty.json";
        let path_to_file2 = "json_output.json";

        let mut object = athena::Object::new();
        object.insert("key1".to_string(), XffValue::from("value1"));
        object.insert("key2".to_string(), XffValue::from(2));
        let json_value1 = XffValue::Object(object);

        write_pretty(path_to_file1, MawuContents::Json(json_value1), 4)
            .expect("Failed to write JSON file");

        let json_value2 = XffValue::from(vec![
            XffValue::from("a"),
            XffValue::from(1),
            XffValue::from(vec![XffValue::from(-1), XffValue::from(true)]),
        ]);
        write(path_to_file2, MawuContents::Json(json_value2)).expect("Failed to write JSON file");

        let read_json1 = read::json(path_to_file1).unwrap();
        let read_json2 = read::json(path_to_file2).unwrap();

        assert!(read_json1.is_object());
        assert_eq!(
            read_json1
                .into_object()
                .unwrap()
                .get("key1")
                .unwrap()
                .into_string()
                .unwrap(),
            "value1"
        );
        assert_eq!(
            read_json1
                .into_object()
                .unwrap()
                .get("key2")
                .unwrap()
                .into_number()
                .unwrap()
                .into_usize()
                .unwrap(),
            2
        );

        assert!(read_json2.is_array());
        assert_eq!(read_json2.into_array().unwrap().len(), 3);
        assert_eq!(
            read_json2
                .into_array()
                .unwrap()
                .get(0)
                .unwrap()
                .into_string()
                .unwrap(),
            "a"
        );

        std::fs::remove_file(path_to_file1).unwrap();
        std::fs::remove_file(path_to_file2).unwrap();
    }

    #[test]
    #[cfg(feature = "csv")]
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
        assert_eq!(
            read_csv.as_csv_object().unwrap()[0]
                .get("key1")
                .unwrap()
                .into_string()
                .unwrap(),
            "value1"
        );

        std::fs::remove_file(path_to_file).unwrap();
    }
}
