use core::fmt;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
/// MawuValue wraps all data types supported by Mawu.
/// It can be constructed using the `MawuValue::from` function on almost any basic rust type,
/// including Option's, Vector's and HashMap's.
/// Using the `MawuValue::default` or `MawuValue::new` function will return an `MawuValue::None`.
pub enum MawuValue {
    CSVObject(Vec<HashMap<String, MawuValue>>),
    CSVArray(Vec<Vec<MawuValue>>),
    Object(HashMap<String, MawuValue>),
    Array(Vec<MawuValue>),
    Uint(u64),
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

impl fmt::Display for MawuValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MawuValue::CSVObject(ref v) => write!(f, "{:?}", v),
            MawuValue::CSVArray(ref v) => write!(f, "{:?}", v),
            MawuValue::Object(ref v) => write!(f, "{:?}", v),
            MawuValue::Array(ref v) => write!(
                f,
                "{}",
                v.iter()
                    .map(|v| {
                        if v.is_none() {
                            format!("\"None\"")
                        } else {
                            let tmp = v.to_string().expect("Unable to convert MawuValue to String");
                            format!("\"{}\"", tmp)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" , ")
            ),
            MawuValue::Uint(ref v) => write!(f, "{}", v),
            MawuValue::Int(ref v) => write!(f, "{}", v),
            MawuValue::Float(ref v) => write!(f, "{}", v),
            MawuValue::String(ref v) => write!(f, "{}", v),
            MawuValue::Bool(ref v) => write!(f, "{}", v),
            MawuValue::None => write!(f, "None"),
        }
    }
}

#[test]
#[ignore]
fn mawu_value_display_needs_nocapture() {
    let mawu_uint = MawuValue::Uint(1);
    println!("UINT: {}", mawu_uint);

    let mawu_float = MawuValue::Float(1.0);
    println!("FLOAT: {}", mawu_float);

    let mawu_string = MawuValue::String("hello".to_string());
    println!("STRING: {}", mawu_string);

    let mawu_bool = MawuValue::Bool(true);
    println!("BOOL: {}", mawu_bool);

    let mawu_none = MawuValue::None;
    println!("NONE: {}", mawu_none);

    let mawu_array = MawuValue::Array(vec![MawuValue::None, MawuValue::Uint(1)]);
    println!("ARRAY: {}", mawu_array);

    let mawu_object = MawuValue::Object(
        vec![("hello".to_string(), MawuValue::Uint(1))]
            .into_iter()
            .collect(),
    );
    println!("OBJECT: {}", mawu_object);

    let mawu_csv_object = MawuValue::CSVObject(vec![vec![("hello".to_string(), MawuValue::Uint(1))].into_iter().collect()]);
    println!("CSV_OBJECT: {}", mawu_csv_object);

    let mawu_csv_array = MawuValue::CSVArray(vec![vec![MawuValue::Uint(1)]]);
    println!("CSV_ARRAY: {}", mawu_csv_array);

    assert!(true);
}

impl Default for MawuValue {
    fn default() -> Self {
        MawuValue::None
    }
}

impl<V> From<Option<V>> for MawuValue
where
    V: Into<MawuValue>,
{
    fn from(value: Option<V>) -> Self {
        match value {
            Some(v) => v.into(),
            None => MawuValue::None,
        }
    }
}

impl<K, V> From<HashMap<K, V>> for MawuValue
where
    K: Into<String>,
    V: Into<MawuValue>,
{
    fn from(value: HashMap<K, V>) -> Self {
        MawuValue::Object(
            value
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl<T> From<Vec<T>> for MawuValue
where
    T: Into<MawuValue>,
{
    fn from(value: Vec<T>) -> Self {
        MawuValue::Array(value.into_iter().map(|x| x.into()).collect())
    }
}

impl MawuValue {
    /// Creates a new MawuValue::CSVObject with the first vector and hashmap inside initialized and
    /// empty.
    pub fn new_csv_object() -> MawuValue {
        MawuValue::CSVObject(vec![HashMap::new()])
    }

    /// Creates a new MawuValue::CSVArray with the first vector and vector inside initialized and empty.
    pub fn new_csv_array() -> MawuValue {
        MawuValue::CSVArray(vec![Vec::new()])
    }

    /// Creates a new MawuValue::Object with an empty hashmap
    pub fn new_object() -> MawuValue {
        MawuValue::Object(HashMap::new())
    }

    /// Creates a new MawuValue::Array with an empty vector
    pub fn new_array() -> MawuValue {
        MawuValue::Array(Vec::new())
    }
}
#[test]
fn new_array_object() {
    let array = MawuValue::new_array();
    let object = MawuValue::new_object();
    let csv_array = MawuValue::new_csv_array();
    let csv_object = MawuValue::new_csv_object();
    assert_eq!(array, MawuValue::Array(vec![]));
    assert_eq!(object, MawuValue::Object(HashMap::new()));
    assert_eq!(csv_array, MawuValue::CSVArray(vec![vec![]]));
    assert_eq!(csv_object, MawuValue::CSVObject(vec![HashMap::new()]));
}

impl From<usize> for MawuValue {
    fn from(value: usize) -> Self {
        MawuValue::Uint(value as u64)
    }
}

impl From<u64> for MawuValue {
    fn from(value: u64) -> Self {
        MawuValue::Uint(value)
    }
}

impl From<u32> for MawuValue {
    fn from(value: u32) -> Self {
        MawuValue::Uint(value as u64)
    }
}

impl From<u16> for MawuValue {
    fn from(value: u16) -> Self {
        MawuValue::Uint(value as u64)
    }
}

impl From<u8> for MawuValue {
    fn from(value: u8) -> Self {
        MawuValue::Uint(value as u64)
    }
}

impl From<isize> for MawuValue {
    fn from(value: isize) -> Self {
        MawuValue::Int(value as i64)
    }
}

impl From<i64> for MawuValue {
    fn from(value: i64) -> Self {
        MawuValue::Int(value)
    }
}

impl From<i32> for MawuValue {
    fn from(value: i32) -> Self {
        MawuValue::Int(value as i64)
    }
}

impl From<i16> for MawuValue {
    fn from(value: i16) -> Self {
        MawuValue::Int(value as i64)
    }
}

impl From<i8> for MawuValue {
    fn from(value: i8) -> Self {
        MawuValue::Int(value as i64)
    }
}

impl From<f64> for MawuValue {
    fn from(value: f64) -> Self {
        MawuValue::Float(value)
    }
}

impl From<f32> for MawuValue {
    fn from(value: f32) -> Self {
        MawuValue::Float(value as f64)
    }
}

impl From<bool> for MawuValue {
    fn from(value: bool) -> Self {
        MawuValue::Bool(value)
    }
}

impl From<String> for MawuValue {
    fn from(value: String) -> Self {
        if value.is_empty() {
            MawuValue::None
        } else if value.parse::<u64>().is_ok() {
            MawuValue::Uint(value.parse().unwrap())
        } else if value.parse::<i64>().is_ok() {
            MawuValue::Int(value.parse().unwrap())
        } else if value.parse::<f64>().is_ok() {
            let test_bind = value.parse::<f64>().unwrap();
            if test_bind.is_nan() || test_bind.is_infinite() {
                MawuValue::None
            } else {
                MawuValue::Float(value.parse().unwrap())
            }
        } else if value.parse::<bool>().is_ok() {
            MawuValue::Bool(value.parse().unwrap())
        } else {
            MawuValue::String(value)
        }
    }
}

impl From<&String> for MawuValue {
    fn from(value: &String) -> Self {
        if value.is_empty() {
            MawuValue::None
        } else if value.parse::<u64>().is_ok() {
            MawuValue::Uint(value.parse().unwrap())
        } else if value.parse::<i64>().is_ok() {
            MawuValue::Int(value.parse().unwrap())
        } else if value.parse::<f64>().is_ok() {
            let test_bind = value.parse::<f64>().unwrap();
            if test_bind.is_nan() || test_bind.is_infinite() {
                MawuValue::None
            } else {
                MawuValue::Float(value.parse().unwrap())
            }
        } else if value.parse::<bool>().is_ok() {
            MawuValue::Bool(value.parse().unwrap())
        } else {
            MawuValue::String(value.to_string())
        }
    }
}

impl From<&str> for MawuValue {
    fn from(value: &str) -> Self {
        if value.is_empty() {
            MawuValue::None
        } else if value.parse::<u64>().is_ok() {
            MawuValue::Uint(value.parse().unwrap())
        } else if value.parse::<i64>().is_ok() {
            MawuValue::Int(value.parse().unwrap())
        } else if value.parse::<f64>().is_ok() {
            let test_bind = value.parse::<f64>().unwrap();
            if test_bind.is_nan() || test_bind.is_infinite() {
                MawuValue::None
            } else {
                MawuValue::Float(value.parse().unwrap())
            }
        } else if value.parse::<bool>().is_ok() {
            MawuValue::Bool(value.parse().unwrap())
        } else {
            MawuValue::String(value.to_string())
        }
    }
}

impl MawuValue {
    /// To create a new `MawuValue`, please use the `MawuValue::from` function. It works on almost any basic rust type,
    /// including Option's, Vector's and HashMap's.
    /// Using the `MawuValue::default` or `MawuValue::new` function will return an `MawuValue::None`.
    pub fn new() -> Self {
        MawuValue::None
    }
    /// Check if the value is an `CSV-Object`
    ///
    /// ## Returns
    /// `true` if the value is an `CSV-Object`, `false` otherwise.
    pub fn is_csv_object(&self) -> bool {
        match self {
            MawuValue::CSVObject(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an `CSV-Array`
    ///
    /// ## Returns
    /// `true` if the value is an `CSV-Array`, `false` otherwise.
    pub fn is_csv_array(&self) -> bool {
        match self {
            MawuValue::CSVArray(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an object
    ///
    /// ## Returns
    /// `true` if the value is an object, `false` otherwise.
    pub fn is_object(&self) -> bool {
        match self {
            MawuValue::Object(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an array
    ///
    /// ## Returns
    /// `true` if the value is an array, `false` otherwise.
    pub fn is_array(&self) -> bool {
        match self {
            MawuValue::Array(_) => true,
            _ => false,
        }
    }

    /// Check if the value is a string
    ///
    /// ## Returns
    /// `true` if the value is a string, `false` otherwise.
    pub fn is_string(&self) -> bool {
        match self {
            MawuValue::String(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an unsigned integer
    ///
    /// ## Returns
    /// `true` if the value is an unsigned integer, `false` otherwise.
    pub fn is_uint(&self) -> bool {
        match self {
            MawuValue::Uint(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an integer
    ///
    /// ## Returns
    /// `true` if the value is an integer, `false` otherwise.
    pub fn is_int(&self) -> bool {
        match self {
            MawuValue::Int(_) => true,
            _ => false,
        }
    }

    /// Check if the value is a float
    ///
    /// ## Returns
    /// `true` if the value is a float, `false` otherwise.
    pub fn is_float(&self) -> bool {
        match self {
            MawuValue::Float(_) => true,
            _ => false,
        }
    }

    /// Check if the value is a number
    ///
    /// ## Returns
    /// `true` if the value is a number, `false` otherwise.
    pub fn is_number(&self) -> bool {
        match self {
            MawuValue::Uint(_) => true,
            MawuValue::Int(_) => true,
            MawuValue::Float(_) => true,
            _ => false,
        }
    }

    /// Check if the value is a boolean
    ///
    /// ## Returns
    /// `true` if the value is a boolean, `false` otherwise.
    pub fn is_bool(&self) -> bool {
        match self {
            MawuValue::Bool(_) => true,
            _ => false,
        }
    }

    /// Simple convenience method to check if the value is a boolean and `true`.
    pub fn is_true(&self) -> bool {
        match self {
            MawuValue::Bool(v) => match v {
                true => true,
                false => false,
            },
            _ => false,
        }
    }

    /// Simple convenience method to check if the value is a boolean and `false`.
    pub fn is_false(&self) -> bool {
        match self {
            MawuValue::Bool(v) => match v {
                true => false,
                false => true,
            },
            _ => false,
        }
    }

    /// Simple convenience method to check if the value is `None`.
    ///
    /// ## Returns
    /// `true` if the value is `None`, `false` otherwise.
    pub fn is_none(&self) -> bool {
        match self {
            MawuValue::None => true,
            _ => false,
        }
    }

    /// Returns `Some(&Vec<HashMap<String, MawuValue>>)` if the value is an `CSV-Object`, `None` otherwise.
    pub fn as_csv_object(&self) -> Option<&Vec<HashMap<String, MawuValue>>> {
        match self {
            MawuValue::CSVObject(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&Vec<Vec<MawuValue>>)` if the value is an `CSV-Array`, `None` otherwise.
    pub fn as_csv_array(&self) -> Option<&Vec<Vec<MawuValue>>> {
        match self {
            MawuValue::CSVArray(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&HashMap<String, MawuValue>)` if the value is an object, `None` otherwise.
    pub fn as_object(&self) -> Option<&HashMap<String, MawuValue>> {
        match self {
            MawuValue::Object(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&Vec<MawuValue>)` if the value is an array, `None` otherwise.
    pub fn as_array(&self) -> Option<&Vec<MawuValue>> {
        match self {
            MawuValue::Array(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&String)` if the value is a String, `None` otherwise.
    pub fn as_string(&self) -> Option<&String> {
        match self {
            MawuValue::String(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&str)` if the value is a String, `None` otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            MawuValue::String(v) => Some(v.as_str()),
            _ => None,
        }
    }

    /// Returns `Some(&u64)` if the value is an integer, `None` otherwise.
    pub fn as_uint(&self) -> Option<&u64> {
        match self {
            MawuValue::Uint(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&i64)` if the value is an integer, `None` otherwise.
    pub fn as_int(&self) -> Option<&i64> {
        match self {
            MawuValue::Int(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&f64)` if the value is a float, `None` otherwise.
    pub fn as_float(&self) -> Option<&f64> {
        match self {
            MawuValue::Float(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&bool)` if the value is a boolean, `None` otherwise.
    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            MawuValue::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `None` if the value is `None` and `Some(())` otherwise.
    pub fn as_none(&self) -> Option<()> {
        match self {
            MawuValue::None => None,
            _ => Some(()),
        }
    }

    /// Returns a owned copy of the value as an `Vec<HashMap<String, MawuValue>>`.
    /// Returns `None` if the value is not an `CSV-Object`.
    pub fn to_csv_object(&self) -> Option<Vec<HashMap<String, MawuValue>>> {
        match self {
            MawuValue::CSVObject(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `Vec<Vec<MawuValue>>`.
    /// Returns `None` if the value is not a `CSV-Array`.
    pub fn to_csv_array(&self) -> Option<Vec<Vec<MawuValue>>> {
        match self {
            MawuValue::CSVArray(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `HashMap<String, MawuValue>`.
    /// Returns `None` if the value is not an `Object`.
    pub fn to_object(&self) -> Option<HashMap<String, MawuValue>> {
        match self {
            MawuValue::Object(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `Vec<MawuValue>`.
    /// Casts any primitive type representable as an `Array` to an `Array`.
    /// Returns `None` if the value is not a primitive type.
    pub fn to_array(&self) -> Option<Vec<MawuValue>> {
        match self {
            MawuValue::Array(v) => Some(v.clone()),
            MawuValue::String(v) => Some(vec![MawuValue::String(v.clone())]),
            MawuValue::None => Some(vec![MawuValue::None]),
            MawuValue::Int(v) => Some(vec![MawuValue::Int(*v)]),
            MawuValue::Uint(v) => Some(vec![MawuValue::Uint(*v)]),
            MawuValue::Float(v) => Some(vec![MawuValue::Float(*v)]),
            MawuValue::Bool(v) => Some(vec![MawuValue::Bool(*v)]),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as a `String`.
    /// Casts any other primitive type representable as a `String` to a `String`.
    /// Returns `None` if the value is not a primitive type.
    pub fn to_string(&self) -> Option<String> {
        match self {
            MawuValue::String(v) => Some(v.to_string()),
            MawuValue::None => Some("".to_string()),
            MawuValue::Int(v) => Some(v.to_string()),
            MawuValue::Uint(v) => Some(v.to_string()),
            MawuValue::Float(v) => Some(v.to_string()),
            MawuValue::Bool(v) => Some(v.to_string()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as a `u64`.
    /// Casts any other primitive type representable as a `u64` to a `u64`.
    /// Returns `None` if the value is not a number.
    pub fn to_uint(&self) -> Option<u64> {
        match self {
            MawuValue::Uint(v) => Some(*v),
            MawuValue::Int(v) => {
                if v.is_positive() {
                    let tmp = v.to_string().parse::<u64>();
                    if tmp.is_ok() {
                        Some(tmp.unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            MawuValue::Float(v) => {
                if v.is_normal() {
                    let tmp = v.to_string().parse::<u64>();
                    if tmp.is_ok() {
                        Some(tmp.unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `i64`.
    /// Casts any other primitive type representable as an `i64` to an `i64`.
    /// Returns `None` if the value is not a number.
    pub fn to_int(&self) -> Option<i64> {
        match self {
            MawuValue::Int(v) => Some(*v),
            MawuValue::Uint(v) => {
                let tmp = v.to_string().parse::<i64>();
                if tmp.is_ok() {
                    Some(tmp.unwrap())
                } else {
                    None
                }
            }
            MawuValue::Float(v) => {
                if v.is_normal() {
                    let tmp = v.to_string().parse::<i64>();
                    if tmp.is_ok() {
                        Some(tmp.unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns a owned copy of the value as a `f64`.
    /// Casts any other primitive type representable as a `f64` to a `f64`.
    /// Returns `None` if the value is not a number.
    pub fn to_float(&self) -> Option<f64> {
        match self {
            MawuValue::Float(v) => Some(*v),
            MawuValue::Int(v) => {
                let tmp = v.to_string().parse::<f64>();
                if tmp.is_ok() {
                    Some(tmp.unwrap())
                } else {
                    None
                }
            }
            MawuValue::Uint(v) => {
                let tmp = v.to_string().parse::<f64>();
                if tmp.is_ok() {
                    Some(tmp.unwrap())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns a owned copy of the value as a `bool`.
    ///
    /// ## Returns
    /// A owned copy of the value as a `bool`.
    /// `None` if the value is not a boolean and could not be represented as one.
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            MawuValue::Bool(v) => Some(*v),
            // I don't think that this code will ever actually return anything besides `None`
            _ => {
                let tmp = self.to_string();
                if tmp.is_some() {
                    let tmp2 = tmp.unwrap().parse::<bool>();
                    if tmp2.is_ok() {
                        Some(tmp2.unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Returns `None` if the value is `None` and `Some(())` otherwise.
    /// Consider using `is_none` instead.
    pub fn to_none(&self) -> Option<()> {
        match self {
            MawuValue::None => None,
            _ => Some(()),
        }
    }
}

// While not 100% test coverage, it's a decent sanity check

#[test]
fn general_as_all_types() {
    let num_uint = MawuValue::from(u8::MAX);
    assert_eq!(num_uint.as_uint().unwrap(), &255);
    let num_int = MawuValue::from(-123);
    assert_eq!(num_int.as_int().unwrap(), &-123);
    let num_float = MawuValue::from(123.2);
    assert_eq!(num_float.as_float().unwrap(), &123.2);
    let bool = MawuValue::from(true);
    assert_eq!(bool.as_bool().unwrap(), &true);
    let none = MawuValue::from("");
    assert!(none.as_none().is_none());

    let array = MawuValue::from(vec!["test", "test2", "test3"]);
    assert_eq!(array.as_array().unwrap()[2], MawuValue::from("test3"));
    let mut hashmap = HashMap::new();
    hashmap.insert("test".to_string(), MawuValue::from(123));
    let object = MawuValue::Object(hashmap);
    assert_eq!(object.as_object().unwrap().get("test").unwrap(), &MawuValue::from(123));

    let string = MawuValue::from("test");
    assert_eq!(string.as_string().unwrap(), &"test");
    let str_ing = MawuValue::from(String::from("test"));
    assert_eq!(str_ing.as_str().unwrap(), "test");
}

#[test]
fn general_convenience_functions() {
    let num = MawuValue::from(123);
    assert!(num.is_number());
}

#[test]
fn convenience_boolean_methods() {
    let bool_true = MawuValue::Bool(true);
    assert!(bool_true.is_true());
    assert!(!bool_true.is_false());

    let bool_false = MawuValue::Bool(false);
    assert!(!bool_false.is_true());
    assert!(bool_false.is_false());

    let not_bool = MawuValue::from("test");
    assert!(!not_bool.is_true());
    assert!(!not_bool.is_false());
}

#[test]
fn from_vec_and_hashmap() {
    let vec = vec!["test", "test2", "test3"];
    let mawu_vec = MawuValue::from(vec);
    assert_eq!(
        mawu_vec,
        MawuValue::Array(vec!["test".into(), "test2".into(), "test3".into()])
    );

    let hashmap = std::collections::HashMap::from([("test", "test2")]);
    let mawu_hashmap = MawuValue::from(hashmap);
    assert_eq!(
        mawu_hashmap,
        MawuValue::Object(HashMap::from([("test".into(), "test2".into())]))
    );
}

#[test]
fn to_primitive() {
    let mawu = MawuValue::from("test").to_string().unwrap();
    assert_eq!(mawu, "test".to_string());
    let bool_true = MawuValue::from("true").to_bool().unwrap();
    assert_eq!(bool_true, true);
    let bool_false = MawuValue::from("false").to_bool().unwrap();
    assert_eq!(bool_false, false);
}

#[test]
fn as_primitive() {
    let tmp = MawuValue::from("test");
    let mawu_str = tmp.as_str().unwrap();
    assert_eq!(mawu_str, "test");
    let mawu = tmp.as_string().unwrap();
    assert_eq!(mawu, &"test".to_string());
}

#[test]
fn float_inf() {
    let float_inf = MawuValue::from("1.0e500000");
    assert!(float_inf.is_none());
}

#[test]
fn number_conversion() {
    let mawu_int = MawuValue::Int(-123);
    let mawu_uint = MawuValue::Uint(123);
    let mawu_float = MawuValue::Float(123.123);
    let mawu_short_float = MawuValue::Float(123.0);

    // all into u64
    let mawu_int_u64 = mawu_int.to_uint();
    let mawu_uint_u64 = mawu_uint.to_uint();
    let mawu_float_u64 = mawu_float.to_uint();
    let mawu_short_float_u64 = mawu_short_float.to_uint();
    assert!(mawu_int_u64.is_none());
    assert!(mawu_uint_u64.unwrap() == 123);
    assert!(mawu_float_u64.is_none());
    assert!(mawu_short_float_u64.unwrap() == 123);

    // all into i64
    let mawu_int_i64 = mawu_int.to_int();
    let mawu_uint_i64 = mawu_uint.to_int();
    let mawu_float_i64 = mawu_float.to_int();
    let mawu_short_float_i64 = mawu_short_float.to_int();
    assert!(mawu_int_i64.unwrap() == -123);
    assert!(mawu_uint_i64.unwrap() == 123);
    assert!(mawu_float_i64.is_none());
    assert!(mawu_short_float_i64.unwrap() == 123);

    // all into f64
    let mawu_int_f64 = mawu_int.to_float();
    let mawu_uint_f64 = mawu_uint.to_float();
    let mawu_float_f64 = mawu_float.to_float();
    let mawu_short_float_f64 = mawu_short_float.to_float();
    assert!(mawu_int_f64.unwrap() == -123.0);
    assert!(mawu_uint_f64.unwrap() == 123.0);
    assert!(mawu_float_f64.unwrap() == 123.123);
    assert!(mawu_short_float_f64.unwrap() == 123.0);
}

#[test]
fn mawu_value_from_string() {
    let mawu_string_value = MawuValue::from("test");
    assert_eq!(mawu_string_value, MawuValue::String("test".to_string()));
    assert_eq!(mawu_string_value.is_string(), true);
    assert_eq!(mawu_string_value.as_string(), Some(&"test".to_string()));

    let mawu_int_value = MawuValue::from("123");
    assert_eq!(mawu_int_value, MawuValue::Uint(123));
    assert_eq!(mawu_int_value.is_uint(), true);
    assert_eq!(mawu_int_value.as_uint(), Some(&123));

    let mawu_int_value = MawuValue::from("-123");
    assert_eq!(mawu_int_value, MawuValue::Int(-123));
    assert_eq!(mawu_int_value.is_int(), true);
    assert_eq!(mawu_int_value.as_int(), Some(&-123));

    let mawu_float_value = MawuValue::from("123.456");
    assert_eq!(mawu_float_value, MawuValue::Float(123.456));
    assert_eq!(mawu_float_value.is_float(), true);
    assert_eq!(mawu_float_value.as_float(), Some(&123.456));

    let mawu_bool_true_value = MawuValue::from("true");
    assert_eq!(mawu_bool_true_value, MawuValue::Bool(true));
    assert_eq!(mawu_bool_true_value.is_bool(), true);
    assert_eq!(mawu_bool_true_value.as_bool(), Some(&true));

    let mawu_bool_false_value = MawuValue::from("false");
    assert_eq!(mawu_bool_false_value, MawuValue::Bool(false));
    assert_eq!(mawu_bool_false_value.is_bool(), true);
    assert_eq!(mawu_bool_false_value.as_bool(), Some(&false));

    let mawu_null_value = MawuValue::from("");
    assert_eq!(mawu_null_value, MawuValue::None);
    assert_eq!(mawu_null_value.is_none(), true);
    assert_eq!(mawu_null_value.as_none(), None);
}

#[test]
fn mawu_value_constructed() {
    let mawu_object_value = MawuValue::Object(HashMap::new());
    let mawu_array_value = MawuValue::Array(vec![]);
    let mawu_csv_object_value = MawuValue::CSVObject(vec![HashMap::new()]);
    let mawu_csv_array_value = MawuValue::CSVArray(vec![vec![]]);

    assert_eq!(mawu_object_value.is_object(), true);
    assert_eq!(mawu_array_value.is_array(), true);
    assert_eq!(mawu_csv_object_value.is_csv_object(), true);
    assert_eq!(mawu_csv_array_value.is_csv_array(), true);

    assert_eq!(mawu_object_value.as_object(), Some(&HashMap::new()));
    assert_eq!(mawu_array_value.as_array(), Some(&vec![]));
    assert_eq!(
        mawu_csv_object_value.as_csv_object(),
        Some(&vec![HashMap::new()])
    );
    assert_eq!(mawu_csv_array_value.as_csv_array(), Some(&vec![vec![]]));
}
