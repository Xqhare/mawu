use crate::{
    MawuError,
    utils::make_whitespace,
};
use athena::{Number, XffValue};

pub fn serialize_json(value: XffValue, spaces: u8, depth: u16) -> Result<String, MawuError> {
    let mut out: String = Default::default();
    let current_whitespace = (spaces as usize).saturating_mul(depth as usize);
    let next_depth = depth.saturating_add(1);
    let next_whitespace = (spaces as usize).saturating_mul(next_depth as usize);
    let is_pretty = spaces > 0;
    match value {
        XffValue::Object(o) => {
            if is_pretty {
                out.push('\n');
            }
            out.push_str(format!("{}{{", make_whitespace(current_whitespace)).as_str());
            if is_pretty {
                out.push('\n');
            }
            for (key, value) in &o {
                out.push_str(format!("{}\"{}\":", make_whitespace(next_whitespace), key).as_str());
                if is_pretty {
                    out.push(' ');
                }
                out.push_str(serialize_json(value.clone(), spaces, next_depth)?.trim_start());
                out.push(',');
                if is_pretty {
                    out.push('\n');
                }
            }
            out = {
                if is_pretty {
                    out.trim_end_matches(",\n").to_string()
                } else {
                    out.trim_end_matches(',').to_string()
                }
            };
            if is_pretty {
                out.push('\n');
                out.push_str(format!("{}}}", make_whitespace(current_whitespace)).as_str());
            } else {
                out.push('}');
            }
        }
        XffValue::Array(a) => {
            if is_pretty {
                out.push('\n');
            }
            out.push_str(format!("{}[", make_whitespace(current_whitespace)).as_str());
            if is_pretty {
                out.push('\n');
                out.push_str(format!("{} ", make_whitespace(next_whitespace)).as_str());
            }
            for v in &a {
                out.push_str(&serialize_json(v.clone(), spaces, next_depth)?);
                out.push(',');
                if is_pretty {
                    out.push(' ');
                }
            }
            out = {
                if is_pretty {
                    out.trim_end_matches(", ").to_string()
                } else {
                    out.trim_end_matches(',').to_string()
                }
            };
            if is_pretty {
                out.push('\n');
                out.push_str(format!("{}]", make_whitespace(current_whitespace)).as_str());
            } else {
                out.push(']');
            }
        }
        XffValue::Null => {
            out.push_str("null");
        }
        XffValue::Boolean(b) => {
            out.push_str(format!("{b}").as_str());
        }
        XffValue::Number(n) => match n {
            Number::Unsigned(u) => out.push_str(format!("{u}").as_str()),
            Number::Integer(i) => out.push_str(format!("{i}").as_str()),
            Number::Float(f) => {
                if f.fract() == 0.0 || f.fract() == -0.0 {
                    out.push_str(&format!("{}{}.0", make_whitespace(spaces), f));
                } else {
                    out.push_str(&format!("{}{}", make_whitespace(spaces), f));
                }
            }
        },
        XffValue::String(s) => {
            out.push_str(serialize_string_to_json(&s.to_string()).as_str());
        }
        XffValue::OrderedObject(o) => {
            if is_pretty {
                out.push('\n');
            }
            out.push_str(format!("{}{{", make_whitespace(current_whitespace)).as_str());
            if is_pretty {
                out.push('\n');
            }
            for (key, value) in o.iter() {
                out.push_str(format!("{}\"{}\":", make_whitespace(next_whitespace), key).as_str());
                if is_pretty {
                    out.push(' ');
                }
                out.push_str(serialize_json(value.clone(), spaces, next_depth)?.trim_start());
                out.push(',');
                if is_pretty {
                    out.push('\n');
                }
            }
            out = {
                if is_pretty {
                    out.trim_end_matches(",\n").to_string()
                } else {
                    out.trim_end_matches(',').to_string()
                }
            };
            if is_pretty {
                out.push('\n');
                out.push_str(format!("{}}}", make_whitespace(current_whitespace)).as_str());
            } else {
                out.push('}');
            }
        }
        XffValue::Table(t) => {
            let mut array = Vec::new();
            for row in &t.rows {
                let mut row_map = std::collections::BTreeMap::new();
                for (i, col_name) in t.columns.iter().enumerate() {
                    if let Some(val) = row.get(i) {
                        row_map.insert(col_name.clone(), val.clone());
                    }
                }
                array.push(XffValue::Object(athena::Object { map: row_map }));
            }
            out.push_str(&serialize_json(
                XffValue::Array(athena::Array { values: array }),
                spaces,
                depth,
            )?);
        }
        XffValue::Metadata(m) => {
            out.push_str(&serialize_json(XffValue::Object(m.map), spaces, depth)?);
        }
        XffValue::DateTime(dt) => {
            out.push_str(&format!("{dt}"));
        }
        XffValue::Duration(d) => {
            out.push_str(&format!("{d}"));
        }
        XffValue::Uuid(u) => {
            out.push_str(&format!("\"{u}\""));
        }
        XffValue::NaN | XffValue::Infinity | XffValue::NegInfinity => {
            out.push_str("null");
        }
        XffValue::Data(d) => {
            // Data is not standard JSON, but we can serialize it as an array of hex values
            out.push('[');
            for (i, byte) in d.data.iter().enumerate() {
                if i != 0 {
                    out.push_str(", ");
                }
                out.push_str(&format!("\"0x{:02x}\"", byte));
            }
            out.push(']');
        }
        XffValue::CommandCharacter(c) => {
            out.push_str(&format!("\"0x{:02x}\"", c.as_u8()));
        }
        XffValue::ArrayCmdChar(ac) => {
            out.push('[');
            for (i, c) in ac.iter().enumerate() {
                if i != 0 {
                    out.push_str(", ");
                }
                out.push_str(&format!("\"0x{:02x}\"", c.as_u8()));
            }
            out.push(']');
        }
        XffValue::Ascii(s) => {
            out.push_str(serialize_string_to_json(&s.to_string()).as_str());
        }
        XffValue::HpFloat(hpf) => {
            out.push_str(&format!("{hpf}"));
        }
        XffValue::LocalDate(d) => {
            out.push_str(&format!("\"{d}\""));
        }
        XffValue::LocalTime(t) => {
            out.push_str(&format!("\"{t}\""));
        }
        XffValue::LocalDateTime(dt) => {
            out.push_str(&format!("\"{dt}\""));
        }
        XffValue::PNan | XffValue::NNan => {
            out.push_str("null");
        }
        _ => {
            out.push_str("null");
        }
    }
    if depth == 0 {
        out = out.trim_start().to_string();
    }
    Ok(out)
}

fn serialize_string_to_json(value: &str) -> String {
    let mut tmp_bind: String = Default::default();
    for (index, c) in value.chars().enumerate() {
        if c == '"' {
            tmp_bind.push_str("\\\"");
        } else if c == '\\' {
            tmp_bind.push('\\');
            if index + 1 == value.len() {
                tmp_bind.push('\\');
            }
        } else if c == '/' {
            tmp_bind.push('\\');
            tmp_bind.push('/');
        } else if c == '\n' {
            tmp_bind.push_str("\\n");
        } else if c == '\r' {
            tmp_bind.push_str("\\r");
        } else if c == '\t' {
            tmp_bind.push_str("\\t");
        } else {
            tmp_bind.push(c);
        }
    }
    format!("\"{tmp_bind}\"")
}
