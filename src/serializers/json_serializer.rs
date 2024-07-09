use crate::{errors::{json_error::{JsonError, JsonWriteError}, MawuError}, mawu_value::MawuValue, utils::make_whitespace};

pub fn serialize_json(value: MawuValue, spaces: u8, depth: u16) -> Result<String, MawuError> {
    let mut out: String = Default::default();
    let current_whitespace = (spaces as usize).saturating_mul(depth as usize);
    let next_depth = depth.saturating_add(1);
    let next_whitespace = (spaces as usize).saturating_mul(next_depth as usize);
    let is_pretty = spaces > 0;
    match value {
        MawuValue::Object(o) => {
            if is_pretty {
                out.push('\n');
            }
            out.push_str(format!("{}{{", make_whitespace(current_whitespace)).as_str());
            if is_pretty {
                out.push('\n');
            }
            for (key, value) in o {
                out.push_str(format!("{}\"{}\":", make_whitespace(next_whitespace), key).as_str());
                if is_pretty {
                    out.push(' ');
                }
                out.push_str(&serialize_json(value, spaces, next_depth)?.trim_start());
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
            
        },
        MawuValue::Array(a) => {
            if is_pretty {
                out.push('\n');
            }
            out.push_str(format!("{}[", make_whitespace(current_whitespace)).as_str());
            if is_pretty {
                out.push('\n');
                out.push_str(format!("{} ", make_whitespace(next_whitespace)).as_str());
            }
            for v in a {
                out.push_str(&serialize_json(v, spaces, next_depth)?);
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
        },
        MawuValue::None => {
            out.push_str("null");
        },
        MawuValue::Bool(b) => {
            out.push_str(format!("{}", b).as_str());
        },
        MawuValue::Uint(u) => {
            out.push_str(format!("{}", u).as_str());
        },
        MawuValue::Int(i) => {
            out.push_str(format!("{}", i).as_str());
        },
        MawuValue::Float(f) => {
            out.push_str(format!("{}", f).as_str());
        },
        MawuValue::String(s) => {
            out.push_str(format!("\"{}\"", s).as_str());
        },
        MawuValue::CSVObject(_) => {
            Err(MawuError::JsonError(JsonError::WriteError(JsonWriteError::NotJSONType("CSVObject".to_string()))))?
        },
        MawuValue::CSVArray(_) => {
            Err(MawuError::JsonError(JsonError::WriteError(JsonWriteError::NotJSONType("CSVArray".to_string()))))?
        },
    };
    if depth == 0 {
        out = out.trim_start().to_string();
    }
    Ok(out)
}
