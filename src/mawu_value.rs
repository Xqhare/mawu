use core::fmt;
use std::collections::HashMap;
use athena::XffValue;

#[derive(Clone, Debug, PartialEq)]
/// MawuValue wraps CSV data types supported by Mawu.
/// Using the `XffValue` from `athena` for the actual data.
pub enum MawuValue {
    /// Only used to hold a headed CSV file
    CSVObject(Vec<HashMap<String, XffValue>>),
    /// Only used to hold a headless CSV file
    CSVArray(Vec<Vec<XffValue>>),
}

impl fmt::Display for MawuValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MawuValue::CSVObject(ref v) => write!(f, "{:?}", v),
            MawuValue::CSVArray(ref v) => write!(f, "{:?}", v),
        }
    }
}

impl MawuValue {
    /// Used only to create a new `MawuValue::CSVObject` you want to fill yourself
    ///
    /// Creates a new `MawuValue::CSVObject` with the first vector and hashmap inside initialized and
    /// empty.
    ///
    /// To unwrap, use `.to_csv_object()`
    pub fn new_csv_object() -> MawuValue {
        MawuValue::CSVObject(vec![HashMap::new()])
    }

    /// Used only to create a new `MawuValue::CSVArray` you want to fill yourself
    ///
    /// Creates a new `MawuValue::CSVArray` with the first vector and vector inside initialized and empty.
    ///
    /// To unwrap, use `.to_csv_array()`
    pub fn new_csv_array() -> MawuValue {
        MawuValue::CSVArray(vec![Vec::new()])
    }

    /// Check if the value is an `CSV-Object`
    pub fn is_csv_object(&self) -> bool {
        match self {
            MawuValue::CSVObject(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an `CSV-Array`
    pub fn is_csv_array(&self) -> bool {
        match self {
            MawuValue::CSVArray(_) => true,
            _ => false,
        }
    }

    /// Returns `Some(&Vec<HashMap<String, XffValue>>)` if the value is an `CSV-Object`, `None` otherwise.
    pub fn as_csv_object(&self) -> Option<&Vec<HashMap<String, XffValue>>> {
        match self {
            MawuValue::CSVObject(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&Vec<Vec<XffValue>>)` if the value is an `CSV-Array`, `None` otherwise.
    pub fn as_csv_array(&self) -> Option<&Vec<Vec<XffValue>>> {
        match self {
            MawuValue::CSVArray(v) => Some(v),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `Vec<HashMap<String, XffValue>>`.
    /// Returns `None` if the value is not an `CSV-Object`.
    pub fn to_csv_object(&self) -> Option<Vec<HashMap<String, XffValue>>> {
        match self {
            MawuValue::CSVObject(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `Vec<Vec<XffValue>>`.
    /// Returns `None` if the value is not a `CSV-Array`.
    pub fn to_csv_array(&self) -> Option<Vec<Vec<XffValue>>> {
        match self {
            MawuValue::CSVArray(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Clears the value
    pub fn clear(&mut self) {
        match self {
            MawuValue::CSVObject(v) => v.clear(),
            MawuValue::CSVArray(v) => v.clear(),
        }
    }

    /// Returns the length of the value
    pub fn len(&self) -> usize {
        match self {
            MawuValue::CSVObject(v) => v.len(),
            MawuValue::CSVArray(v) => v.len(),
        }
    }

    /// Convenience method to check if the value is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            MawuValue::CSVObject(v) => v.is_empty(),
            MawuValue::CSVArray(v) => v.is_empty(),
        }
    }
}

#[test]
fn new_array_object() {
    let csv_array = MawuValue::new_csv_array();
    let csv_object = MawuValue::new_csv_object();
    assert!(csv_array.is_csv_array());
    assert!(csv_object.is_csv_object());
}

#[test]
fn creating_csv_object() {
    let a_hashmap = HashMap::from([("key1".to_string(), XffValue::from(u8::MAX))]);
    let mawu_value = MawuValue::CSVObject(vec![a_hashmap]);
    assert!(mawu_value.is_csv_object());
}

#[test]
fn creating_csv_array() {
    let mawu_value = MawuValue::CSVArray(vec![vec![XffValue::from(u8::MAX)]]);
    assert!(mawu_value.is_csv_array());
}
