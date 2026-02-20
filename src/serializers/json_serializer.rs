use athena::{XffValue, Number};
use crate::{errors::{json_error::{JsonError, JsonWriteError}, MawuError}, utils::make_whitespace};

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
            for (key, value) in o.iter() {
                out.push_str(format!("{}\"{}\":", make_whitespace(next_whitespace), key).as_str());
                if is_pretty {
                    out.push(' ');
                }
                out.push_str(&serialize_json(value.clone(), spaces, next_depth)?.trim_start());
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
        XffValue::Array(a) => {
            if is_pretty {
                out.push('\n');
            }
            out.push_str(format!("{}[", make_whitespace(current_whitespace)).as_str());
            if is_pretty {
                out.push('\n');
                out.push_str(format!("{} ", make_whitespace(next_whitespace)).as_str());
            }
            for v in a.iter() {
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
        },
        XffValue::Null => {
            out.push_str("null");
        },
        XffValue::Boolean(b) => {
            out.push_str(format!("{}", b).as_str());
        },
        XffValue::Number(n) => {
            match n {
                Number::Unsigned(u) => out.push_str(format!("{}", u).as_str()),
                Number::Integer(i) => out.push_str(format!("{}", i).as_str()),
                Number::Float(f) => {
                    if f.fract() == 0.0 || f.fract() == -0.0 {
                        out.push_str(&format!("{}{}.0", make_whitespace(spaces), f));
                    } else {
                        out.push_str(&format!("{}{}", make_whitespace(spaces), f));
                    }
                }
            }
        },
        XffValue::String(s) => {
            out.push_str(serialize_string_to_json(&s).as_str());
        },
        XffValue::Data(d) => {
            // Data is not standard JSON, but we can serialize it as an array of bytes
            out.push('[');
            for (i, byte) in d.iter().enumerate() {
                if i != 0 {
                    out.push_str(", ");
                }
                out.push_str(&format!("{}", byte));
            }
            out.push(']');
        },
        XffValue::CommandCharacter(_) | XffValue::ArrayCmdChar(_) => {
             Err(MawuError::JsonError(JsonError::WriteError(JsonWriteError::NotJSONType("CommandCharacter".to_string()))))?
        }
    };
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
            tmp_bind.push_str("\\");
            if index + 1 == value.len() {
                tmp_bind.push_str("\\");
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
    format!("\"{}\"", tmp_bind)
}
