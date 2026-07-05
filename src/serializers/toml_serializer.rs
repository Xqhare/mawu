use crate::{errors::toml_error::TomlWriteError, utils::make_whitespace};
use athena::{Array, Number, Object, XffValue};
use nemesis::NemesisError;

pub fn serialize_toml(value: XffValue, spaces: u8) -> Result<String, NemesisError> {
    let mut out: String = Default::default();
    let value = match value {
        XffValue::Object(obj) => obj,
        _ => {
            return Err(NemesisError::new(
                "mawu::serializers::toml_serializer::serialize_toml",
                TomlWriteError::ParentMustBeObject,
            ));
        }
    };
    let mut path = Vec::new();
    serialize_object_to_toml(&value, spaces, &mut path, &mut out)?;
    Ok(out)
}

fn is_complex_value(value: &XffValue) -> bool {
    match value {
        XffValue::Object(_)
        | XffValue::OrderedObject(_)
        | XffValue::Table(_)
        | XffValue::Metadata(_) => true,
        XffValue::Array(arr) => {
            if let Some(first) = arr.iter().next() {
                is_complex_value(first)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn escape_toml_string(value: &str) -> String {
    let mut escaped = String::new();
    for c in value.chars() {
        match c {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{0008}' => escaped.push_str("\\b"),
            '\u{000c}' => escaped.push_str("\\f"),
            c if c.is_control() => {
                escaped.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => escaped.push(c),
        }
    }
    escaped
}

fn is_bare_key(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }
    key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn format_toml_key(key: &str) -> String {
    if is_bare_key(key) {
        key.to_string()
    } else {
        format!("\"{}\"", escape_toml_string(key))
    }
}

fn format_toml_path(path: &[String]) -> String {
    path.iter()
        .map(|p| format_toml_key(p))
        .collect::<Vec<String>>()
        .join(".")
}

fn serialize_simple_value_to_toml_string(value: &XffValue) -> Result<String, NemesisError> {
    match value {
        XffValue::Null => Err(NemesisError::new(
            "mawu::serializers::toml_serializer::serialize_simple_value",
            TomlWriteError::NotTOMLType("Null values are not supported in TOML".to_string()),
        )),
        XffValue::Ascii(s) | XffValue::String(s) => Ok(format!("\"{}\"", escape_toml_string(&s.to_string()))),
        XffValue::Boolean(b) => Ok(b.to_string()),
        XffValue::Number(num) => match num {
            Number::Unsigned(u) => Ok(u.to_string()),
            Number::Integer(i) => Ok(i.to_string()),
            Number::Float(f) => Ok(f.to_string()),
        },
        XffValue::HpFloat(hpf) => {
            let v = hpf.get_value();
            let s = hpf.get_scale();
            let num = v as f64 * 10f64.powf(s as f64);
            Ok(num.to_string())
        }
        XffValue::Uuid(uuid) => Ok(uuid.to_string()),
        XffValue::DateTime(dt) => Ok(dt.to_string()),
        XffValue::LocalDateTime(ldt) => Ok(ldt.to_string()),
        XffValue::LocalTime(lt) => Ok(lt.to_string()),
        XffValue::LocalDate(ld) => Ok(ld.to_string()),
        XffValue::Duration(d) => Ok(d.to_string()),
        XffValue::NaN => Ok("nan".to_string()),
        XffValue::NegNaN => Ok("-nan".to_string()),
        XffValue::PosNaN => Ok("+nan".to_string()),
        XffValue::Infinity => Ok("inf".to_string()),
        XffValue::NegInfinity => Ok("-inf".to_string()),
        _ => Err(NemesisError::new(
            "mawu::serializers::toml_serializer::serialize_simple_value",
            TomlWriteError::NotTOMLType(format!("Value type {:?} cannot be serialized as a simple value", value)),
        )),
    }
}

fn serialize_array_to_toml(
    arr: &Array,
    key: &str,
    spaces: u8,
    depth: u16,
    out: &mut String,
) -> Result<(), NemesisError> {
    let current_whitespace = (spaces as usize).saturating_mul(depth as usize);
    let next_depth = depth.saturating_add(1);
    let next_whitespace = (spaces as usize).saturating_mul(next_depth as usize);

    let is_pretty = spaces > 0;
    
    out.push_str(&make_whitespace(current_whitespace));
    out.push_str(&format_toml_key(key));
    out.push_str(" = [");

    if is_pretty && !arr.is_empty() {
        out.push('\n');
    }

    for (i, v) in arr.iter().enumerate() {
        if is_pretty {
            out.push_str(&make_whitespace(next_whitespace));
        }
        let serialized = serialize_simple_value_to_toml_string(v)?;
        out.push_str(&serialized);
        
        if i < arr.len() - 1 {
            out.push(',');
            if is_pretty {
                out.push('\n');
            } else {
                out.push(' ');
            }
        }
    }

    if is_pretty && !arr.is_empty() {
        out.push('\n');
        out.push_str(&make_whitespace(current_whitespace));
    }
    out.push_str("]\n");
    Ok(())
}

fn serialize_object_to_toml(
    obj: &Object,
    spaces: u8,
    path: &mut Vec<String>,
    out: &mut String,
) -> Result<(), NemesisError> {
    let mut simple_fields = Vec::new();
    let mut complex_fields = Vec::new();

    for (k, v) in obj.iter() {
        if is_complex_value(v) {
            complex_fields.push((k, v));
        } else {
            simple_fields.push((k, v));
        }
    }

    // Sort alphabetically for deterministic testing
    simple_fields.sort_by_key(|&(k, _)| k);
    complex_fields.sort_by_key(|&(k, _)| k);

    let depth = path.len() as u16;
    let current_whitespace = (spaces as usize).saturating_mul(depth as usize);

    // 1. Serialize simple fields
    for (k, v) in simple_fields {
        if matches!(v, XffValue::Null) {
            return Err(NemesisError::new(
                "mawu::serializers::toml_serializer::serialize_object_to_toml",
                TomlWriteError::NotTOMLType("Null values are not supported in TOML".to_string()),
            ));
        }
        let formatted_key = format_toml_key(k);
        match v {
            XffValue::Array(arr) => {
                serialize_array_to_toml(arr, k, spaces, depth, out)?;
            }
            _ => {
                let serialized = serialize_simple_value_to_toml_string(v)?;
                out.push_str(&make_whitespace(current_whitespace));
                out.push_str(&formatted_key);
                out.push_str(" = ");
                out.push_str(&serialized);
                out.push('\n');
            }
        }
    }

    // 2. Serialize complex fields
    for (k, v) in complex_fields {
        path.push(k.to_string());
        let header_depth = path.len() as u16;
        let header_whitespace = (spaces as usize).saturating_mul((header_depth - 1) as usize);

        match v {
            XffValue::Object(sub_obj) => {
                out.push_str(&make_whitespace(header_whitespace));
                out.push_str(&format!("[{}]\n", format_toml_path(path)));
                serialize_object_to_toml(sub_obj, spaces, path, out)?;
            }
            XffValue::OrderedObject(ord_obj) => {
                let mut obj = Object::new();
                for (sub_k, sub_v) in ord_obj.iter() {
                    obj.insert(sub_k.clone(), sub_v.clone());
                }
                out.push_str(&make_whitespace(header_whitespace));
                out.push_str(&format!("[{}]\n", format_toml_path(path)));
                serialize_object_to_toml(&obj, spaces, path, out)?;
            }
            XffValue::Metadata(meta) => {
                out.push_str(&make_whitespace(header_whitespace));
                out.push_str(&format!("[{}]\n", format_toml_path(path)));
                serialize_object_to_toml(meta.as_object(), spaces, path, out)?;
            }
            XffValue::Table(table) => {
                let mut index = 0;
                while let Some(row) = table.get_row(index) {
                    index = index.saturating_add(1);
                    out.push_str(&make_whitespace(header_whitespace));
                    out.push_str(&format!("[[{}]]\n", format_toml_path(path)));
                    let ord_obj = row
                        .as_ordered_object()
                        .expect("Row is always returned as an OrderedObject");
                    
                    let mut row_obj = Object::new();
                    for (sub_k, sub_v) in ord_obj.iter() {
                        row_obj.insert(sub_k.clone(), sub_v.clone());
                    }
                    serialize_object_to_toml(&row_obj, spaces, path, out)?;
                }
            }
            XffValue::Array(arr) => {
                // Complex array (array of tables/objects)
                for row_val in arr.iter() {
                    out.push_str(&make_whitespace(header_whitespace));
                    out.push_str(&format!("[[{}]]\n", format_toml_path(path)));
                    match row_val {
                        XffValue::Object(sub_obj) => {
                            serialize_object_to_toml(sub_obj, spaces, path, out)?;
                        }
                        XffValue::OrderedObject(ord_obj) => {
                            let mut row_obj = Object::new();
                            for (sub_k, sub_v) in ord_obj.iter() {
                                row_obj.insert(sub_k.clone(), sub_v.clone());
                            }
                            serialize_object_to_toml(&row_obj, spaces, path, out)?;
                        }
                        XffValue::Metadata(meta) => {
                            serialize_object_to_toml(meta.as_object(), spaces, path, out)?;
                        }
                        _ => {
                            return Err(NemesisError::new(
                                "mawu::serializers::toml_serializer::serialize_object_to_toml",
                                TomlWriteError::NotTOMLType("Array of tables must contain only objects/tables".to_string()),
                            ));
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        path.pop();
    }

    Ok(())
}
