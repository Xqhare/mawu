use crate::{
    errors::{
        csv_error::{CsvError, CsvWriteError},
    },
    mawu_value::MawuValue,
    utils::make_whitespace,
};
use athena::{Number, XffString, XffValue};

fn serialize_csv_string(value: XffString, spaces: u8) -> Result<String, nemesis::NemesisError> {
    let value = value.to_string();
    let mut out = format!("{}\"", make_whitespace(spaces));
    let tmp = value.replace('"', "\"\"");
    out.push_str(&tmp);
    out.push('"');
    Ok(out)
}

fn serialize_csv_value(value: XffValue, spaces: u8) -> Result<String, nemesis::NemesisError> {
    match value {
        XffValue::String(s) => serialize_csv_string(s, spaces),
        XffValue::Number(n) => match n {
            Number::Unsigned(u) => Ok(format!("{}{}", make_whitespace(spaces), u)),
            Number::Integer(i) => Ok(format!("{}{}", make_whitespace(spaces), i)),
            Number::Float(f) => {
                if f.fract() == 0.0 {
                    Ok(format!("{}{}.0", make_whitespace(spaces), f))
                } else {
                    Ok(format!("{}{}", make_whitespace(spaces), f))
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
        XffValue::Object(o) => {
            let mut out = format!("{}{{", make_whitespace(spaces));
            for (i, (k, v)) in o.iter().enumerate() {
                if i != 0 {
                    out.push(',');
                }
                out.push_str(&format!("{}:{}", k, serialize_csv_value(v.clone(), spaces)?));
            }
            out.push('}');
            Ok(out)
        }
        XffValue::OrderedObject(o) => {
            let mut out = format!("{}{{(ordered) ", make_whitespace(spaces));
            for (i, (k, v)) in o.iter().enumerate() {
                if i != 0 {
                    out.push(',');
                }
                out.push_str(&format!("{}:{}", k, serialize_csv_value(v.clone(), spaces)?));
            }
            out.push('}');
            Ok(out)
        }
        XffValue::Table(t) => {
            let mut out = format!("{}Table(cols:[", make_whitespace(spaces));
            for (i, col) in t.columns.iter().enumerate() {
                if i != 0 {
                    out.push(',');
                }
                out.push_str(col);
            }
            out.push_str("],rows:[");
            for (i, row) in t.rows.iter().enumerate() {
                if i != 0 {
                    out.push(',');
                }
                out.push('[');
                for (j, val) in row.iter().enumerate() {
                    if j != 0 {
                        out.push(',');
                    }
                    out.push_str(&serialize_csv_value(val.clone(), spaces)?);
                }
                out.push(']');
            }
            out.push_str("])");
            Ok(out)
        }
        XffValue::Null => Ok(String::new()),
        XffValue::DateTime(dt) => Ok(format!("{}{}", make_whitespace(spaces), dt)),
        XffValue::Duration(d) => Ok(format!("{}{}", make_whitespace(spaces), d)),
        XffValue::Uuid(u) => Ok(format!("{}{}", make_whitespace(spaces), u)),
        XffValue::NaN => Ok(format!("{}NaN", make_whitespace(spaces))),
        XffValue::Infinity => Ok(format!("{}Infinity", make_whitespace(spaces))),
        XffValue::NegInfinity => Ok(format!("{}-Infinity", make_whitespace(spaces))),
        XffValue::Metadata(m) => serialize_csv_value(XffValue::Object(m.map), spaces),
        XffValue::Data(d) => {
            let mut out = format!("{}[", make_whitespace(spaces));
            for (i, b) in d.data.iter().enumerate() {
                if i != 0 {
                    out.push(',');
                }
                out.push_str(&format!("\"0x{:02x}\"", b));
            }
            out.push(']');
            Ok(out)
        }
        XffValue::CommandCharacter(c) => Ok(format!("{}\"0x{:02x}\"", make_whitespace(spaces), c.as_u8())),
        XffValue::ArrayCmdChar(ac) => {
            let mut out = format!("{}[", make_whitespace(spaces));
            for (i, c) in ac.iter().enumerate() {
                if i != 0 {
                    out.push(',');
                }
                out.push_str(&format!("\"0x{:02x}\"", c.as_u8()));
            }
            out.push(']');
            Ok(out)
        }
        XffValue::Ascii(s) => serialize_csv_string(s, spaces),
        XffValue::HpFloat(hpf) => Ok(format!("{}{}", make_whitespace(spaces), hpf)),
        XffValue::LocalDate(d) => Ok(format!("{}{}", make_whitespace(spaces), d)),
        XffValue::LocalTime(t) => Ok(format!("{}{}", make_whitespace(spaces), t)),
        XffValue::LocalDateTime(dt) => Ok(format!("{}{}", make_whitespace(spaces), dt)),
        XffValue::PNan | XffValue::NNan => Ok(format!("{}NaN", make_whitespace(spaces))),
        _ => Err(nemesis::NemesisError::new(
            "mawu::serializers::csv_serializer",
            CsvError::WriteError(CsvWriteError::UnallowedType(format!(
                "Unallowed type for CSV serialization"
            ))),
        )),
    }
}

pub fn serialize_csv_headed(value: MawuValue, spaces: u8) -> Result<String, nemesis::NemesisError> {
    // Headed: Vec<HashMap<String, XffValue>> | Object | OrderedObject | Table

    match value {
        MawuValue::CSVObject(maps) => {
            let mut head_created = false;
            let mut head: String = Default::default();
            let mut body: Vec<String> = Default::default();
            let mut keys: Vec<String> = Default::default();

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
            let mut out = format!("{head}\n");
            out.push_str(body.join("\n").as_str());
            Ok(out)
        }
        MawuValue::Object(o) => {
            let mut head = String::new();
            let mut row = String::new();
            for (i, (k, v)) in o.iter().enumerate() {
                if i != 0 {
                    head.push(',');
                    row.push(',');
                }
                head.push_str(make_whitespace(spaces).as_str());
                head.push_str(k);
                row.push_str(&serialize_csv_value(v.clone(), spaces)?);
            }
            Ok(format!("{head}\n{row}"))
        }
        MawuValue::OrderedObject(o) => {
            let mut head = String::new();
            let mut row = String::new();
            for (i, (k, v)) in o.iter().enumerate() {
                if i != 0 {
                    head.push(',');
                    row.push(',');
                }
                head.push_str(make_whitespace(spaces).as_str());
                head.push_str(k);
                row.push_str(&serialize_csv_value(v.clone(), spaces)?);
            }
            Ok(format!("{head}\n{row}"))
        }
        MawuValue::Table(t) => {
            let mut head = String::new();
            for (i, col) in t.columns.iter().enumerate() {
                if i != 0 {
                    head.push(',');
                }
                head.push_str(make_whitespace(spaces).as_str());
                head.push_str(col);
            }
            let mut body = Vec::new();
            for r in t.rows {
                let mut row = String::new();
                for (i, val) in r.iter().enumerate() {
                    if i != 0 {
                        row.push(',');
                    }
                    row.push_str(&serialize_csv_value(val.clone(), spaces)?);
                }
                body.push(row);
            }
            let mut out = format!("{head}\n");
            out.push_str(body.join("\n").as_str());
            Ok(out)
        }
        _ => Err(nemesis::NemesisError::new(
            "mawu::serializers::csv_serializer",
            CsvError::WriteError(CsvWriteError::UnallowedType(
                "Expected a headed CSV type!".to_string(),
            )),
        )),
    }
}

pub fn serialize_csv_unheaded(value: MawuValue, spaces: u8) -> Result<String, nemesis::NemesisError> {
    // Input == Vec<Vec<XffValue>>
    let rows = if let MawuValue::CSVArray(v) = value {
        v
    } else {
        return Err(nemesis::NemesisError::new(
            "mawu::serializers::csv_serializer",
            CsvError::WriteError(CsvWriteError::UnallowedType(
                "Not a MawuValue::CSVArray!".to_string(),
            )),
        ));
    };

    let mut out = make_whitespace(spaces).clone();
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
