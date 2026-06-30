use athena::{
    Array, LocalDate, LocalDateTime, LocalTime, Number, Object, XffValue, tvec_to_xff_value, xff,
};
use horae::Utc;
use nemesis::NemesisError;

use crate::errors::toml_error::TomlParseError;

pub fn toml_lexer(chars: Vec<String>) -> Result<XffValue, NemesisError> {
    let mut index = 0;
    // Used for error reporting
    let mut line_number = 1;
    if chars.is_empty() {
        return Err(NemesisError::new(
            "mawu::lexers::toml_lexer",
            TomlParseError::UnexpectedEndOfFile,
        ));
    }
    let mut out = Object::new();
    while &index < &chars.len()
        && let Some(char) = chars.get(index)
    {
        if is_whitespace_or_comment(char) {
            skip_whitespace_or_comments(&chars, &mut index, &mut line_number);
            continue;
        }
        if char == "[" {
            let array_of_tables = {
                if &chars[index.saturating_add(1)] == "[" {
                    index = index.saturating_add(1);
                    true
                } else {
                    false
                }
            };
            let table_keys = parse_keys(&chars, &mut index);
            if chars[index] != "]" {
                return Err(NemesisError::new(
                    "mawu::lexers::toml_lexer",
                    TomlParseError::UnexpectedCharacter(format!(
                        "Expected ']' to close table name, but got: '{}'",
                        chars[index]
                    )),
                )
                .add_ctx(format!(
                    "Got table name or names if dotted: '{table_keys:?}'"
                ))
                .add_ctx(format!("Line number: {line_number}")));
            }
            index = {
                if array_of_tables {
                    index.saturating_add(2)
                } else {
                    index.saturating_add(1)
                }
            };
            if is_whitespace_or_comment(&chars[index]) {
                skip_whitespace_or_comments(&chars, &mut index, &mut line_number);
            }
            let value = parse_toml_value(&chars, &mut index, &mut line_number)?;
            todo!("Parse contents of table");
            continue;
        } else {
            let keys = parse_keys(&chars, &mut index);
            if keys.is_empty() {
                return Err(NemesisError::new(
                    "mawu::lexers::toml_lexer",
                    TomlParseError::ExpectedKey,
                )
                .add_ctx(format!("Line number: {line_number}")));
            }
            let value = parse_toml_value(&chars, &mut index, &mut line_number)?;
            let last_key_idx = keys.len() - 1;
            let mut current = &mut out;
            for (i, key) in keys.iter().enumerate() {
                if i == last_key_idx {
                    if current.get(key).is_some() {
                        return Err(NemesisError::new(
                            "mawu::lexers::toml_lexer",
                            TomlParseError::KeyAlreadyDefined,
                        )
                        .add_ctx(format!("Line number: {line_number}")));
                    }
                    current.insert(key.clone(), value);
                    break;
                } else {
                    if current.get(key).is_none() {
                        current.insert(key.clone(), XffValue::Object(Object::new()));
                    }
                    if let Some(next_obj) = current.get_mut(key).and_then(|v| v.as_object_mut()) {
                        current = next_obj;
                    } else {
                        return Err(NemesisError::new(
                            "mawu::lexers::toml_lexer",
                            TomlParseError::KeyAlreadyDefined,
                        )
                        .add_ctx(format!("Line number: {line_number}")));
                    }
                }
            }
        }
        index = index.saturating_add(1);
    }
    Ok(out.into())
}

fn handle_value_equals_sign(
    chars: &Vec<String>,
    index: &mut usize,
    line_number: &mut usize,
) -> Result<(), NemesisError> {
    if is_toml_whitespace_no_newlines(&chars[*index]) {
        skip_whitespace_only(&chars, index);
    } else if chars[*index] == "=" {
        *index = index.saturating_add(1);
        if is_toml_whitespace_no_newlines(&chars[*index]) {
            skip_whitespace_only(&chars, index);
        }
    } else {
        return Err(NemesisError::new(
            "mawu::lexers::toml_lexer",
            TomlParseError::UnexpectedCharacter(format!(
                "Expected '=' or surrounding whitespace, but got: '{}'",
                chars[*index]
            )),
        )
        .add_ctx(format!("Line number: {line_number}")));
    }
    if is_newline(&chars[*index], &chars[index.saturating_add(1)]).0 {
        return Err(NemesisError::new(
            "mawu::lexers::toml_lexer",
            TomlParseError::UnexpectedNewline,
        )
        .add_ctx("Key requires a value.")
        .add_ctx(format!("Line number: {line_number}")));
    }
    Ok(())
}

fn parse_toml_value(
    chars: &Vec<String>,
    index: &mut usize,
    line_number: &mut usize,
) -> Result<XffValue, NemesisError> {
    handle_value_equals_sign(chars, index, line_number)?;

    let mut out: Option<XffValue> = None;
    while index.saturating_add(1) < chars.len() {
        if is_newline(&chars[*index], &chars[index.saturating_add(1)]).0 {
            *index = index.saturating_add(1);
            break;
        }
        if &chars[*index] == "t" || &chars[*index] == "f" {
            parse_toml_bool(chars, index, &mut out, *line_number)?;
        } else if &chars[*index] == "\"" || &chars[*index] == "'" {
            parse_toml_string(chars, index, line_number, &mut out)?;
        } else if &chars[*index] == "n"
            || &chars[*index] == "i"
            || &chars[*index] == "+"
            || &chars[*index] == "-"
            || is_number(&chars[*index])
        {
            parse_toml_number_or_datetime(chars, index, &mut out, *line_number)?;
        } else if &chars[*index] == "[" {
            *index = index.saturating_add(1);
            parse_toml_array(chars, index, &mut out, line_number)?;
        } else if &chars[*index] == "{" {
            parse_toml_table(chars, index, &mut out, line_number)?;
        } else {
            return Err(NemesisError::new(
                "mawu::lexers::toml_lexer",
                TomlParseError::UnexpectedCharacter(format!(
                    "Expected a value, but got: '{}'",
                    chars[*index]
                )),
            )
            .add_ctx(format!("Line number: {line_number}")));
        }
        *index = index.saturating_add(1);
    }
    Ok(out.unwrap())
}

fn parse_toml_table(
    chars: &Vec<String>,
    index: &mut usize,
    out: &mut Option<XffValue>,
    line_number: &mut usize,
) -> Result<(), NemesisError> {
    if index.saturating_add(1) >= chars.len() {
        return Err(NemesisError::new(
            "mawu::lexers::toml_lexer::parse_toml_table",
            TomlParseError::UnexpectedEndOfFile,
        )
        .add_ctx("Inside an inline table, but reached the end of the file.")
        .add_ctx(format!("Line number: {line_number}")));
    }
    let mut object = Object::new();
    let (is_whitespace, skip, skip_lines) =
        is_toml_whitespace(&chars[*index], &chars[index.saturating_add(1)]);
    if is_whitespace {
        *index = index.saturating_add(skip);
        *line_number = line_number.saturating_add(skip_lines);
    }
    if &chars[*index] == "}" {
        *index = index.saturating_add(1);
        out.insert(xff!(object));
        return Ok(());
    }
    while index.saturating_add(1) < chars.len() {
        let keys = parse_keys(chars, index);
        let value = parse_toml_value(chars, index, line_number)?;
        object.insert(keys[0].clone(), value);
        let (is_whitespace, skip, skip_lines) =
            is_toml_whitespace(&chars[*index], &chars[index.saturating_add(1)]);
        if is_whitespace {
            *index = index.saturating_add(skip);
            *line_number = line_number.saturating_add(skip_lines);
        }
        if &chars[*index] == "}" {
            *index = index.saturating_add(1);
            break;
        } else if &chars[*index] == "," {
            *index = index.saturating_add(1);
            continue;
        } else {
            return Err(NemesisError::new(
                "mawu::lexers::toml_lexer::parse_toml_table",
                TomlParseError::UnexpectedCharacter(format!(
                    "Expected a value, but got: '{}'",
                    chars[*index]
                )),
            )
            .add_ctx(format!("Line number: {line_number}")));
        }
    }
    out.insert(xff!(object));
    Ok(())
}

fn parse_toml_array(
    chars: &Vec<String>,
    index: &mut usize,
    out: &mut Option<XffValue>,
    line_number: &mut usize,
) -> Result<(), NemesisError> {
    if index.saturating_add(1) >= chars.len() {
        return Err(NemesisError::new(
            "mawu::lexers::toml_lexer",
            TomlParseError::UnexpectedEndOfFile,
        )
        .add_ctx("Inside an array, but reached the end of the file.")
        .add_ctx(format!("Line number: {line_number}")));
    }
    let mut array = Array::new();
    let (it_whitespace, skip, skip_lines) =
        is_toml_whitespace(&chars[*index], &chars[index.saturating_add(1)]);
    if it_whitespace {
        *index = index.saturating_add(skip);
        *line_number = line_number.saturating_add(skip_lines);
    }
    if &chars[*index] == "]" {
        *index = index.saturating_add(1);
        out.insert(xff!(array));
        return Ok(());
    }
    while index.saturating_add(1) < chars.len() {
        if &chars[*index] == "]" {
            *index = index.saturating_add(1);
            break;
        }
        array.push(parse_toml_value(chars, index, line_number)?);
        if &chars[*index] == "," {
            *index = index.saturating_add(1);
        }
        let (is_whitespace, skip, skip_lines) =
            is_toml_whitespace(&chars[*index], &chars[index.saturating_add(1)]);
        if is_whitespace {
            *index = index.saturating_add(skip);
            *line_number = line_number.saturating_add(skip_lines);
        }
    }
    out.insert(XffValue::Array(array));
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringType {
    Single,
    Double,
    TripleSingle,
    TripleDouble,
}

fn parse_toml_string(
    chars: &Vec<String>,
    index: &mut usize,
    line_number: &mut usize,
    out: &mut Option<XffValue>,
) -> Result<(), NemesisError> {
    let str_type = {
        if &chars[*index] == "'" {
            if &chars[index.saturating_add(1)] == "'"
                && &chars[index.saturating_add(2)] == "'"
                && &chars[index.saturating_add(3)] == "'"
            {
                *index = index.saturating_add(3);
                StringType::TripleSingle
            } else {
                *index = index.saturating_add(1);
                StringType::Single
            }
        } else if &chars[*index] == "\"" {
            if &chars[index.saturating_add(1)] == "\""
                && &chars[index.saturating_add(2)] == "\""
                && &chars[index.saturating_add(3)] == "\""
            {
                *index = index.saturating_add(3);
                StringType::TripleDouble
            } else {
                *index = index.saturating_add(1);
                StringType::Double
            }
        } else {
            return Err(NemesisError::new(
                "mawu::lexers::toml_lexer::parse_toml_string",
                TomlParseError::UnexpectedCharacter(format!("Expected a string, but got: '{}'", chars[*index])),
            ).add_ctx("Passed in invalid string type - Ensure it starts with either a single ' or \"; Or a triple \"\"\" or '''"));
        }
    };
    let mut tmp_buf = String::with_capacity(64); // Arbitrary pre allocation

    while index.saturating_add(1) < chars.len() {
        match str_type {
            StringType::Single => {
                if &chars[*index] == "'" {
                    *index = index.saturating_add(1);
                    break;
                } else {
                    tmp_buf.push_str(&chars[*index]);
                }
            }
            StringType::Double => {
                handle_escaped_sequences(chars, index, &mut tmp_buf)?;
                if &chars[*index] == "\"" {
                    *index = index.saturating_add(1);
                    break;
                } else {
                    tmp_buf.push_str(&chars[*index]);
                }
            }
            StringType::TripleSingle => {
                handle_multiline_cont(chars, index, line_number, &mut tmp_buf);
                if &chars[*index] == "'"
                    && index.saturating_add(4) < chars.len()
                    && &chars[index.saturating_add(1)] == "'"
                    && &chars[index.saturating_add(2)] == "'"
                    && is_toml_whitespace(
                        &chars[index.saturating_add(3)],
                        &chars[index.saturating_add(4)],
                    )
                    .0
                {
                    *index = index.saturating_add(3);
                    break;
                } else {
                    tmp_buf.push_str(&chars[*index]);
                }
            }
            StringType::TripleDouble => {
                handle_multiline_cont(chars, index, line_number, &mut tmp_buf);
                handle_escaped_sequences(chars, index, &mut tmp_buf)?;
                if &chars[*index] == "\""
                    && index.saturating_add(3) < chars.len()
                    && &chars[index.saturating_add(1)] == "\""
                    && &chars[index.saturating_add(2)] == "\""
                {
                    *index = index.saturating_add(3);
                    break;
                }
            }
        }
    }
    *out = Some(XffValue::from(tmp_buf));
    Ok(())
}

fn handle_multiline_cont(
    chars: &Vec<String>,
    index: &mut usize,
    line_number: &mut usize,
    tmp_buf: &mut String,
) {
    if &chars[*index] == "\\" && index.saturating_add(2) < chars.len() {
        let (is_whitespace, skip, skip_lines) =
            is_toml_whitespace(&chars[*index], &chars[index.saturating_add(1)]);
        if is_whitespace {
            tmp_buf.push('\n');
            *index = index.saturating_add(skip);
            *line_number = line_number.saturating_add(skip_lines);
        }
    }
}

fn handle_escaped_sequences(
    chars: &Vec<String>,
    index: &mut usize,
    tmp_buf: &mut String,
) -> Result<(), NemesisError> {
    if index.saturating_add(1) < chars.len() && is_single_escaped_char(chars, index) {
        *index = index.saturating_add(2);
        tmp_buf.push_str(&chars[*index]);
        tmp_buf.push_str(&chars[index.saturating_add(1)]);
    } else if &chars[*index] == "\\" && index.saturating_add(1) < chars.len() {
        match chars[index.saturating_add(1)].as_str() {
            "x" => {
                if index.saturating_add(3) < chars.len() && is_hex_digit_range(chars, index, 2) {
                    // U+00xx
                    let mut u8: [u8; 2] = [0; 2];
                    let hex = format!(
                        "{}{}",
                        chars[index.saturating_add(2)],
                        chars[index.saturating_add(3)]
                    );
                    u8[1] = u8::from_str_radix(&hex, 16).unwrap();
                    tmp_buf.push_str(&String::from_utf8_lossy(&u8));
                    *index = index.saturating_add(4);
                } else {
                    return Err(NemesisError::new(
                                    "mawu::lexers::toml_lexer::parse_toml_string",
                                    TomlParseError::UnexpectedCharacter(format!("Expected an escaped unicode identifier, but got: '{}{}'", chars[index.saturating_add(2)], chars[index.saturating_add(3)]))
                                ).add_ctx("Passed in invalid unicode escape sequence - Ensure it starts with '\\x' and is followed by two hex digits"));
                }
            }
            "u" => {
                if index.saturating_add(5) < chars.len() && is_hex_digit_range(chars, index, 4) {
                    // U+xxxx
                    let mut u8 = [0; 2];
                    let hex1 = format!(
                        "{}{}",
                        chars[index.saturating_add(2)],
                        chars[index.saturating_add(3)]
                    );
                    u8[0] = u8::from_str_radix(&hex1, 16).unwrap();
                    let hex2 = format!(
                        "{}{}",
                        chars[index.saturating_add(4)],
                        chars[index.saturating_add(5)]
                    );
                    u8[1] = u8::from_str_radix(&hex2, 16).unwrap();
                    tmp_buf.push_str(&String::from_utf8_lossy(&u8));
                    *index = index.saturating_add(6);
                } else {
                    return Err(NemesisError::new(
                                    "mawu::lexers::toml_lexer::parse_toml_string",
                                    TomlParseError::UnexpectedCharacter(format!("Expected an escaped unicode identifier, but got: '{}{}{}{}'", chars[index.saturating_add(2)], chars[index.saturating_add(3)], chars[index.saturating_add(4)], chars[index.saturating_add(5)]))
                                ).add_ctx("Passed in invalid unicode escape sequence - Ensure it starts with '\\u' and is followed by four hex digits"));
                }
            }
            "U" => {
                if index.saturating_add(9) < chars.len() && is_hex_digit_range(chars, index, 8) {
                    // U+xxxxxx
                    let mut u8 = [0; 4];
                    let hex1 = format!(
                        "{}{}",
                        chars[index.saturating_add(2)],
                        chars[index.saturating_add(3)]
                    );
                    u8[0] = u8::from_str_radix(&hex1, 16).unwrap();
                    let hex2 = format!(
                        "{}{}",
                        chars[index.saturating_add(4)],
                        chars[index.saturating_add(5)]
                    );
                    u8[1] = u8::from_str_radix(&hex2, 16).unwrap();
                    let hex3 = format!(
                        "{}{}",
                        chars[index.saturating_add(6)],
                        chars[index.saturating_add(7)]
                    );
                    u8[2] = u8::from_str_radix(&hex3, 16).unwrap();
                    let hex4 = format!(
                        "{}{}",
                        chars[index.saturating_add(8)],
                        chars[index.saturating_add(9)]
                    );
                    u8[3] = u8::from_str_radix(&hex4, 16).unwrap();
                    tmp_buf.push_str(&String::from_utf8_lossy(&u8));
                    *index = index.saturating_add(10);
                } else {
                    return Err(NemesisError::new(
                                    "mawu::lexers::toml_lexer::parse_toml_string",
                                    TomlParseError::UnexpectedCharacter(format!("Expected an escaped unicode identifier, but got: '{}{}{}{}{}{}{}{}'", chars[index.saturating_add(2)], chars[index.saturating_add(3)], chars[index.saturating_add(4)], chars[index.saturating_add(5)],chars[index.saturating_add(6)], chars[index.saturating_add(7)], chars[index.saturating_add(8)], chars[index.saturating_add(9)] ))
                                ).add_ctx("Passed in invalid unicode escape sequence - Ensure it starts with '\\U' and is followed by eight hex digits"));
                }
            }
            _ => {
                return Err(NemesisError::new(
                    "mawu::lexers::toml_lexer::parse_toml_string",
                    TomlParseError::UnexpectedCharacter(format!(
                        "Expected escaped unicode identifier (\\x, \\u, \\U), but got: '{}{}'",
                        chars[*index],
                        chars[index.saturating_add(1)]
                    )),
                ));
            }
        }
    }
    Ok(())
}

fn is_hex_digit_range(chars: &Vec<String>, index: &mut usize, until_index: usize) -> bool {
    if index.saturating_add(until_index) < chars.len() {
        for i in 1..until_index {
            if !is_hex_digit(&chars[index.saturating_add(i)]) {
                return false;
            }
        }
        true
    } else {
        false
    }
}

fn is_hex_digit(s: &str) -> bool {
    for c in s.chars() {
        if c.is_ascii_hexdigit() {
            return true;
        }
    }
    false
}

/// Checks for `\\` followed by `\"`, `\\`, `\\b`, `\\t`, `\\n`, `\\f`, `\\r`, `\\e`
///
/// Returns true if the next chars match, false if not
fn is_single_escaped_char(chars: &Vec<String>, index: &mut usize) -> bool {
    if &chars[*index] == "\\" && index.saturating_add(1) < chars.len() {
        if &chars[index.saturating_add(1)] == "\""
            || &chars[index.saturating_add(1)] == "\\"
            || &chars[index.saturating_add(1)] == "b"
            || &chars[index.saturating_add(1)] == "t"
            || &chars[index.saturating_add(1)] == "n"
            || &chars[index.saturating_add(1)] == "f"
            || &chars[index.saturating_add(1)] == "r"
            || &chars[index.saturating_add(1)] == "e"
        {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn parse_toml_number_or_datetime(
    chars: &Vec<String>,
    index: &mut usize,
    out: &mut Option<XffValue>,
    line_number: usize,
) -> Result<(), NemesisError> {
    let mut tmp_buf = String::new();
    if chars[*index] == "+" || chars[*index] == "-" {
        tmp_buf.push_str(&chars[*index]);
        *index = index.saturating_add(1);
    }
    while index.saturating_add(1) < chars.len() {
        if is_end_of_value(chars, index) {
            break;
        } else {
            // just drop the underscore, accept everything else
            if &chars[*index] != "_" {
                tmp_buf.push_str(&chars[*index]);
            }
        }
    }
    if tmp_buf == "+"
        || tmp_buf == "-"
        || (tmp_buf.starts_with("0") && tmp_buf.len() != 1)
        || tmp_buf.starts_with(".")
        || tmp_buf.ends_with(".")
    {
        return Err(NemesisError::new(
            "mawu::lexers::toml_lexer",
            TomlParseError::UnexpectedCharacter(format!(
                "Expected a number, but got: '{}'",
                chars[*index]
            )),
        )
        .add_ctx("Numbers cannot only consist of a + or -.")
        .add_ctx(format!("Line number: {line_number}")));
    }
    if tmp_buf.contains("inf") || tmp_buf.contains("nan") {
        match tmp_buf.as_str() {
            "inf" | "+inf" => {
                *out = Some(XffValue::Infinity);
                return Ok(());
            }
            "-inf" => {
                *out = Some(XffValue::NegInfinity);
                return Ok(());
            }
            "nan" => {
                *out = Some(XffValue::NaN);
                return Ok(());
            }
            "+nan" => {
                *out = Some(XffValue::PosNaN);
                return Ok(());
            }
            "-nan" => {
                *out = Some(XffValue::NegNaN);
                return Ok(());
            }
            _ => {
                return Err(NemesisError::new(
                    "mawu::lexers::toml_lexer",
                    TomlParseError::UnexpectedCharacter(format!(
                        "Expected either an infinity (+inf, inf or -inf) or a NaN (+nan, nan or -nan), but got: '{}'",
                        chars[*index]
                    )),
                )
                .add_ctx("Infinities and NaNs are case sensitive and may only have a leading sign.")
                .add_ctx(format!("Line number: {line_number}")));
            }
        }
    }
    if tmp_buf.starts_with("0x") {
        if (tmp_buf.len() <= 4 && tmp_buf.len() >= 3)
            && let Ok(value) = u8::from_str_radix(&tmp_buf[2..], 16)
        {
            *out = Some(XffValue::from(value));
            return Ok(());
        } else {
            return Err(NemesisError::new(
                "mawu::lexers::toml_lexer",
                TomlParseError::UnexpectedCharacter(format!(
                    "Expected a hex number, but got: '{}'",
                    chars[*index]
                )),
            )
            .add_ctx("Hex numbers always start with '0x'.")
            .add_ctx(format!("Line number: {line_number}")));
        }
    } else if tmp_buf.starts_with("0b") {
        if (tmp_buf.len() <= 10 && tmp_buf.len() >= 3)
            && let Ok(value) = u8::from_str_radix(&tmp_buf[2..], 2)
        {
            *out = Some(XffValue::from(value));
            return Ok(());
        } else {
            return Err(NemesisError::new(
                "mawu::lexers::toml_lexer",
                TomlParseError::UnexpectedCharacter(format!(
                    "Expected a binary number, but got: '{}'",
                    chars[*index]
                )),
            )
            .add_ctx("Binary numbers always start with '0b'.")
            .add_ctx(format!("Line number: {line_number}")));
        }
    } else if tmp_buf.starts_with("0o") {
        if (tmp_buf.len() <= 4 && tmp_buf.len() >= 3)
            && let Ok(value) = u8::from_str_radix(&tmp_buf[2..], 8)
        {
            *out = Some(XffValue::from(value));
            return Ok(());
        } else {
            return Err(NemesisError::new(
                "mawu::lexers::toml_lexer",
                TomlParseError::UnexpectedCharacter(format!(
                    "Expected an octal number, but got: '{}'",
                    chars[*index]
                )),
            )
            .add_ctx("Octal numbers always start with '0o'.")
            .add_ctx(format!("Line number: {line_number}")));
        }
    }
    if let Some(date_time) = Utc::from_rfc3339(&tmp_buf) {
        *out = Some(date_time.to_xffvalue());
        return Ok(());
    }
    if let Ok(local_date_time) = LocalDateTime::try_from(tmp_buf.as_str()) {
        *out = Some(xff!(local_date_time));
        return Ok(());
    }
    if let Ok(local_date) = LocalDate::try_from(tmp_buf.as_str()) {
        *out = Some(xff!(local_date));
        return Ok(());
    }
    if let Ok(local_time) = LocalTime::try_from(tmp_buf.as_str()) {
        *out = Some(xff!(local_time));
        return Ok(());
    }
    if let Ok(number) = Number::try_from(tmp_buf.as_str()) {
        *out = Some(xff!(number));
        return Ok(());
    }
    Err(NemesisError::new(
        "mawu::lexers::toml_lexer",
        TomlParseError::UnexpectedCharacter(format!(
            "Expected a number, but got: '{}'",
            chars[*index]
        )),
    )
    .add_ctx("Unable to parse given stream as a number, or local_date_time, local_time or local_date, even though it looks like one.")
    .add_ctx(format!("Line number: {line_number}")))
}

fn is_end_of_value(chars: &Vec<String>, index: &mut usize) -> bool {
    if chars.len() <= *index {
        true
    } else {
        is_toml_whitespace(&chars[*index], &chars[index.saturating_add(1)]).0
            || &chars[*index] == "]"
            || &chars[*index] == "}"
            || &chars[*index] == ","
    }
}

/// Returns true if the string is a number (0-9)
fn is_number(s: &str) -> bool {
    s.chars().all(char::is_numeric)
}

fn parse_toml_bool(
    chars: &Vec<String>,
    index: &mut usize,
    out: &mut Option<XffValue>,
    line_number: usize,
) -> Result<(), NemesisError> {
    if &chars[*index] == "t"
        && index.saturating_add(3) < chars.len()
        && &chars[*index + 1] == "r"
        && &chars[*index + 2] == "u"
        && &chars[*index + 3] == "e"
    {
        *out = Some(XffValue::from(true));
        Ok(())
    } else if &chars[*index] == "f"
        && index.saturating_add(4) < chars.len()
        && &chars[*index + 1] == "a"
        && &chars[*index + 2] == "l"
        && &chars[*index + 3] == "s"
        && &chars[*index + 4] == "e"
    {
        *out = Some(XffValue::from(false));
        Ok(())
    } else {
        Err(NemesisError::new(
            "mawu::lexers::toml_lexer",
            TomlParseError::UnexpectedCharacter(format!(
                "Expected a boolean, but got: '{}'",
                chars[*index]
            )),
        )
        .add_ctx(format!("Line number: {line_number}")))
    }
}

/// Returns the parsed key.
/// Should the key be dotted, it will return all the keys separated by dots
///
/// Handles `[[table.name.key]]`; Stops at the first `]` (among other things)
fn parse_keys(chars: &Vec<String>, index: &mut usize) -> Vec<String> {
    let mut kind = KeyKind::Bare;
    if chars[*index] == "\"" {
        kind = KeyKind::DoubleQuoted;
        *index = index.saturating_add(1);
    } else if chars[*index] == "'" {
        kind = KeyKind::SingleQuoted;
        *index = index.saturating_add(1);
    }
    let mut out: Vec<String> = Default::default();
    let mut key = String::new();
    while index.saturating_add(1) < chars.len() {
        match kind {
            KeyKind::Bare => {
                if is_valid_bare_key_char(&chars[*index]) {
                    key.push_str(&chars[*index]);
                    *index = index.saturating_add(1);
                } else {
                    if is_dotted_key(chars, index) {
                        kind = KeyKind::Dotted;
                        out.push(key);
                        key = String::new();
                        continue;
                    }
                    break;
                }
            }
            KeyKind::Dotted => {
                out.extend(parse_keys(chars, index));
            }
            KeyKind::DoubleQuoted => {
                make_double_quoted_key(chars, index);
            }
            KeyKind::SingleQuoted => {
                make_single_quoted_key(chars, index);
            }
        }
    }
    if !key.is_empty() {
        out.push(key);
    }
    out
}

fn is_dotted_key(chars: &Vec<String>, index: &mut usize) -> bool {
    while index.saturating_add(1) < chars.len() {
        let char = &chars[*index];
        if char == "." {
            *index = index.saturating_add(1);
            return true;
        }
        if is_toml_whitespace_no_newlines(char) {
            skip_whitespace_only(chars, index);
            continue;
        }
        return false;
    }
    false
}

fn is_valid_bare_key_char(s: &str) -> bool {
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
            return true;
        }
    }
    false
}

fn make_single_quoted_key(chars: &Vec<String>, index: &mut usize) -> String {
    make_quoted_key(chars, index, "'")
}

fn make_double_quoted_key(chars: &Vec<String>, index: &mut usize) -> String {
    make_quoted_key(chars, index, "\"")
}

fn make_quoted_key(chars: &Vec<String>, index: &mut usize, pattern: &str) -> String {
    let mut out = String::new();
    while index.saturating_add(1) < chars.len() {
        if &chars[*index] == pattern {
            break;
        }
        out.push_str(&chars[*index]);
        *index = index.saturating_add(1);
    }
    out
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum KeyKind {
    Bare,
    DoubleQuoted,
    SingleQuoted,
    Dotted,
}

/// Checks if the given string is whitespace or a comment
///
/// Also supports double `\n` to skip empty lines
fn is_whitespace_or_comment(s: &str) -> bool {
    is_toml_whitespace(s, "").0 || s == "#"
}

fn skip_whitespace_only(chars: &Vec<String>, index: &mut usize) {
    while index.saturating_add(1) < chars.len() {
        let s = &chars[*index];
        if s == "\t" || s == " " {
            *index = index.saturating_add(1);
        } else {
            break;
        }
    }
}

/// Skips whitespace and comments, updating the index
fn skip_whitespace_or_comments(chars: &Vec<String>, index: &mut usize, skip_newlines: &mut usize) {
    let mut in_comment = false;
    while index.saturating_add(1) < chars.len() {
        if in_comment {
            let (is_newline, skip, skip_lines) =
                is_newline(&chars[*index], &chars[index.saturating_add(1)]);
            *skip_newlines = skip_newlines.saturating_add(skip_lines);
            if is_newline {
                in_comment = false;
            }
            *index = index.saturating_add(skip);
            continue;
        }
        let (is_newline, skip, skip_lines) =
            is_newline(&chars[*index], &chars[index.saturating_add(1)]);
        *skip_newlines = skip_newlines.saturating_add(skip_lines);
        if is_newline {
            *index = index.saturating_add(skip);
            continue;
        }
        let (is_whitespace, skip, skip_lines) =
            is_toml_whitespace(&chars[*index], &chars[index.saturating_add(1)]);
        *skip_newlines = skip_newlines.saturating_add(skip_lines);
        if is_whitespace {
            *index = index.saturating_add(skip);
        } else if &chars[*index] == "#" && !in_comment {
            in_comment = true;
            continue;
        } else {
            break;
        }
    }
}

/// Checks if the given string is whitespace or not
///
/// Does not support newlines
fn is_toml_whitespace_no_newlines(s: &str) -> bool {
    s == "\t" || s == " "
}

/// Checks if the given string is whitespace or not (including newlines)
///
/// Also supports double `\n` to skip empty lines
///
/// # Returns
/// - `(is_whitespace, skip, lines)` -> is_whitespace: bool, skip (in bytes): usize, lines (0-2 if double `\n`): usize
fn is_toml_whitespace(s: &str, next_char: &str) -> (bool, usize, usize) {
    if s == "\t" || s == " " {
        if next_char == "\t" || next_char == " " {
            (true, 2, 0)
        } else {
            (true, 1, 0)
        }
    } else {
        is_newline(s, next_char)
    }
}

/// Matches unix & windows styles for TOML 1.1.0 compliance
///
/// Also supports double `\n` to skip empty lines immediately
///
/// # Returns
/// - `(is_newline, skip, lines)` -> is_newline: bool, skip (in bytes): usize, lines (0-2 if double `\n`): usize
fn is_newline(s: &str, next_char: &str) -> (bool, usize, usize) {
    if (s == "\r" || s == "\n") && next_char == "\n" {
        (true, 2, 1)
    } else if s == "\n" {
        (true, 2, 2)
    } else {
        (false, 0, 0)
    }
}
