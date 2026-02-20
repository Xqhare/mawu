use athena::{XffValue, Number};
use crate::{errors::{csv_error::{CsvError, CsvWriteError}, MawuError}, mawu_value::MawuValue, utils::make_whitespace};

fn serialize_csv_string(value: String, spaces: u8) -> Result<String, MawuError> {
            let mut out = format!("{}\"", make_whitespace(spaces));
            let tmp = value.replace("\"", "\"\"");
            out.push_str(&tmp);
            out.push('"');
            Ok(out)
}

fn serialize_csv_value(value: XffValue, spaces: u8) -> Result<String, MawuError> {
    match value {
        XffValue::String(s) => serialize_csv_string(s, spaces),
        XffValue::Number(n) => {
            match n {
                Number::Unsigned(u) => Ok(format!("{}{}", make_whitespace(spaces), u)),
                Number::Integer(i) => Ok(format!("{}{}", make_whitespace(spaces), i)),
                Number::Float(f) => {
                    if f.fract() == 0.0 {
                        Ok(format!("{}{}.0", make_whitespace(spaces), f))
                    } else {
                       Ok(format!("{}{}", make_whitespace(spaces), f)) 
                    }
                }
            }
        },
        XffValue::Boolean(b) => Ok(format!("{}{}", make_whitespace(spaces), b)),
        XffValue::Array(a) => {
            let mut out = format!("{}[", make_whitespace(spaces));
            for (i, v) in a.iter().enumerate() {
                if i != 0 {
                    out.push(',');
                }
                out.push_str(&serialize_csv_value(v.clone(), spaces)?);
            }
            out.push(']');
            Ok(out)
        }
        XffValue::Null => Ok(String::new()),
        // All other types are not allowed or serialized as something else
        XffValue::Object(_) => Err(MawuError::CsvError(CsvError::WriteError(CsvWriteError::UnallowedType("Object".to_string())))),
        XffValue::Data(_) => Err(MawuError::CsvError(CsvError::WriteError(CsvWriteError::UnallowedType("Data".to_string())))),
        XffValue::CommandCharacter(_) => Err(MawuError::CsvError(CsvError::WriteError(CsvWriteError::UnallowedType("CommandCharacter".to_string())))),
        XffValue::ArrayCmdChar(_) => Err(MawuError::CsvError(CsvError::WriteError(CsvWriteError::UnallowedType("ArrayCmdChar".to_string())))),
    }
}

pub fn serialize_csv_headed(value: MawuValue, spaces: u8) -> Result<String, MawuError> {
    // Headed: Vec<HashMap<String, XffValue>>

    let mut head_created = false;
    let mut head: String = Default::default();
    let mut body: Vec<String> = Default::default();
    let mut keys: Vec<String> = Default::default();
    
    let maps = if let MawuValue::CSVObject(v) = value {
        v
    } else {
        return Err(MawuError::CsvError(CsvError::WriteError(CsvWriteError::UnallowedType("Not a MawuValue::CSVObject!".to_string()))));
    };

    for map in maps {
        let mut row: String = Default::default();
        if !head_created {
            for (i, (key, _)) in map.iter().enumerate() {
                keys.push(key.clone());
                if i != 0 {
                    head.push(',');
                }
                head.push_str(make_whitespace(spaces).as_str());
                head.push_str(key);
            }
            head_created = true;
        }
        for (i, key) in keys.iter().enumerate() {
            if i != 0 {
                row.push(',');
            }
            let get_val = map.get(key).unwrap();
            row.push_str(&serialize_csv_value(get_val.clone(), spaces)?);
        }
        body.push(row);
    }
    let mut out = format!("{}\n", head);
    out.push_str(body.join("\n").as_str());
    Ok(out)
}

pub fn serialize_csv_unheaded(value: MawuValue, spaces: u8) -> Result<String, MawuError> {
    // Input == Vec<Vec<XffValue>>
    let rows = if let MawuValue::CSVArray(v) = value {
        v
    } else {
        return Err(MawuError::CsvError(CsvError::WriteError(CsvWriteError::UnallowedType("Not a MawuValue::CSVArray!".to_string()))));
    };

    let mut out = format!("{}", make_whitespace(spaces));
    for (row_idx, v) in rows.iter().enumerate() {
        if row_idx != 0 {
            out.push('\n');
        }
        let mut row = String::new();
        for (i, val) in v.iter().enumerate() {
            if i != 0 {
                row.push(',');
            }
            row.push_str(&serialize_csv_value(val.clone(), spaces)?);
        }
        out.push_str(&row);
    }
    Ok(out)
}
