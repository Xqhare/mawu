use athena::XffValue;
use core::fmt;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
/// `MawuValue` wraps CSV data types supported by Mawu.
/// Using the `XffValue` from `athena` for the actual data.
pub enum MawuValue {
    /// Only used to hold a headed CSV file
    CSVObject(Vec<HashMap<String, XffValue>>),
    /// Only used to hold a headless CSV file
    CSVArray(Vec<Vec<XffValue>>),
    /// XffValue variant Object
    Object(athena::Object),
    /// XffValue variant OrderedObject
    OrderedObject(athena::OrderedObject),
    /// XffValue variant Table
    Table(athena::Table),
}

impl fmt::Display for MawuValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MawuValue::CSVObject(ref v) => write!(f, "{v:?}"),
            MawuValue::CSVArray(ref v) => write!(f, "{v:?}"),
            MawuValue::Object(ref o) => write!(f, "{o:?}"),
            MawuValue::OrderedObject(ref o) => write!(f, "{o:?}"),
            MawuValue::Table(ref t) => write!(f, "{t:?}"),
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
    #[must_use]
    pub fn new_csv_object() -> MawuValue {
        MawuValue::CSVObject(vec![HashMap::new()])
    }

    /// Used only to create a new `MawuValue::CSVArray` you want to fill yourself
    ///
    /// Creates a new `MawuValue::CSVArray` with the first vector and vector inside initialized and empty.
    ///
    /// To unwrap, use `.to_csv_array()`
    #[must_use]
    pub fn new_csv_array() -> MawuValue {
        MawuValue::CSVArray(vec![Vec::new()])
    }

    /// Check if the value is an `CSV-Object`
    #[must_use]
    pub fn is_csv_object(&self) -> bool {
        matches!(self, MawuValue::CSVObject(_))
    }

    /// Check if the value is an `CSV-Array`
    #[must_use]
    pub fn is_csv_array(&self) -> bool {
        matches!(self, MawuValue::CSVArray(_))
    }

    /// Check if the value is an `Object`
    #[must_use]
    pub fn is_object(&self) -> bool {
        matches!(self, MawuValue::Object(_))
    }

    /// Check if the value is an `OrderedObject`
    #[must_use]
    pub fn is_ordered_object(&self) -> bool {
        matches!(self, MawuValue::OrderedObject(_))
    }

    /// Check if the value is a `Table`
    #[must_use]
    pub fn is_table(&self) -> bool {
        matches!(self, MawuValue::Table(_))
    }

    /// Returns `Some(&Vec<HashMap<String, XffValue>>)` if the value is an `CSV-Object`, `None` otherwise.
    #[must_use]
    pub fn as_csv_object(&self) -> Option<&Vec<HashMap<String, XffValue>>> {
        match self {
            MawuValue::CSVObject(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&Vec<Vec<XffValue>>)` if the value is an `CSV-Array`, `None` otherwise.
    #[must_use]
    pub fn as_csv_array(&self) -> Option<&Vec<Vec<XffValue>>> {
        match self {
            MawuValue::CSVArray(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&athena::Object)` if the value is an `Object`, `None` otherwise.
    #[must_use]
    pub fn as_object(&self) -> Option<&athena::Object> {
        match self {
            MawuValue::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Returns `Some(&athena::OrderedObject)` if the value is an `OrderedObject`, `None` otherwise.
    #[must_use]
    pub fn as_ordered_object(&self) -> Option<&athena::OrderedObject> {
        match self {
            MawuValue::OrderedObject(o) => Some(o),
            _ => None,
        }
    }

    /// Returns `Some(&athena::Table)` if the value is a `Table`, `None` otherwise.
    #[must_use]
    pub fn as_table(&self) -> Option<&athena::Table> {
        match self {
            MawuValue::Table(t) => Some(t),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `Vec<HashMap<String, XffValue>>`.
    /// Returns `None` if the value is not an `CSV-Object`.
    #[must_use]
    pub fn to_csv_object(&self) -> Option<Vec<HashMap<String, XffValue>>> {
        match self {
            MawuValue::CSVObject(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `Vec<Vec<XffValue>>`.
    /// Returns `None` if the value is not a `CSV-Array`.
    #[must_use]
    pub fn to_csv_array(&self) -> Option<Vec<Vec<XffValue>>> {
        match self {
            MawuValue::CSVArray(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `athena::Object`.
    /// Returns `None` if the value is not an `Object`.
    #[must_use]
    pub fn to_object(&self) -> Option<athena::Object> {
        match self {
            MawuValue::Object(o) => Some(o.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `athena::OrderedObject`.
    /// Returns `None` if the value is not an `OrderedObject`.
    #[must_use]
    pub fn to_ordered_object(&self) -> Option<athena::OrderedObject> {
        match self {
            MawuValue::OrderedObject(o) => Some(o.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `athena::Table`.
    /// Returns `None` if the value is not a `Table`.
    #[must_use]
    pub fn to_table(&self) -> Option<athena::Table> {
        match self {
            MawuValue::Table(t) => Some(t.clone()),
            _ => None,
        }
    }

    /// Clears the value
    pub fn clear(&mut self) {
        match self {
            MawuValue::CSVObject(v) => v.clear(),
            MawuValue::CSVArray(v) => v.clear(),
            MawuValue::Object(o) => o.clear(),
            MawuValue::OrderedObject(o) => o.clear(),
            MawuValue::Table(t) => {
                t.columns.clear();
                t.rows.clear();
            }
        }
    }

    /// Returns the length of the value
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            MawuValue::CSVObject(v) => v.len(),
            MawuValue::CSVArray(v) => v.len(),
            MawuValue::Object(o) => o.len(),
            MawuValue::OrderedObject(o) => o.len(),
            MawuValue::Table(t) => t.rows.len(),
        }
    }

    /// Convenience method to check if the value is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            MawuValue::CSVObject(v) => v.is_empty(),
            MawuValue::CSVArray(v) => v.is_empty(),
            MawuValue::Object(o) => o.is_empty(),
            MawuValue::OrderedObject(o) => o.is_empty(),
            MawuValue::Table(t) => t.rows.is_empty(),
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
