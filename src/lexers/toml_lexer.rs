use athena::{Object, XffValue};
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
            index = index.saturating_add(1);
            if is_whitespace_or_comment(&chars[index]) {
                skip_whitespace_or_comments(&chars, &mut index, &mut line_number);
            }
            let value = parse_toml_value(&chars, &mut index, &mut line_number)?;
            todo!("Parse contents of table");
            continue;
        } else {
            let keys = parse_keys(&chars, &mut index);
            let value = parse_toml_value(&chars, &mut index, &mut line_number)?;
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
            parse_toml_bool(chars, index, &mut out);
        } else if &chars[*index] == "\"" || &chars[*index] == "'" {
            parse_toml_string(chars, index, &mut out);
        } else if &chars[*index] == "n"
            || &chars[*index] == "i"
            || &chars[*index] == "+"
            || &chars[*index] == "-"
            || is_number(&chars[*index])
        {
            parse_toml_number_or_datetime(chars, index, &mut out, *line_number)?;
        } else if &chars[*index] == "[" {
            parse_toml_array(chars, index, &mut out);
        } else if &chars[*index] == "{" {
            parse_toml_table(chars, index, &mut out);
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
        if is_number(&chars[*index]) {
            tmp_buf.push_str(&chars[*index]);
            *index = index.saturating_add(1);
            continue;
        } else if chars[*index] == "o" || chars[*index] == "b" || chars[*index] == "x" {
            parse_toml_number_base(chars, index, &mut tmp_buf);
        } else if chars[*index] == "n" || chars[*index] == "i" {
            parse_toml_number_inf_nan(chars, index, &mut tmp_buf);
        } else if chars[*index] == "T"
            || chars[*index] == "Z"
            || chars[*index] == ":"
            || chars[*index] == " "
        {
        } else {
            // Allow +, -, ., e, E (but only one 'e' or 'E' or '.'. Also '+' or '-' only
            // AFTER either 'e' or 'E')
            // Ignore `_`
            tmp_buf.push_str(&chars[*index]);
        }
    }
    if tmp_buf == "+" || tmp_buf == "-" {
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
    *out = Some(XffValue::from(tmp_buf));
    Ok(())
}

/// Returns true if the string is a number (0-9)
fn is_number(s: &str) -> bool {
    s.chars().all(char::is_numeric)
}

fn parse_toml_bool(chars: &Vec<String>, index: &mut usize, out: &mut Option<XffValue>) {
    if &chars[*index] == "t"
        && index.saturating_add(3) < chars.len()
        && &chars[*index + 1] == "r"
        && &chars[*index + 2] == "u"
        && &chars[*index + 3] == "e"
    {
        *out = Some(XffValue::from(true));
    } else if &chars[*index] == "f"
        && index.saturating_add(4) < chars.len()
        && &chars[*index + 1] == "a"
        && &chars[*index + 2] == "l"
        && &chars[*index + 3] == "s"
        && &chars[*index + 4] == "e"
    {
        *out = Some(XffValue::from(false));
    }
}

/// Returns the parsed key.
/// Should the key be dotted, it will return all the keys separated by dots
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

fn parse_table(chars: &Vec<String>, index: &mut usize) -> (String, XffValue) {
    todo!()
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
            let (is_newline, skip) = is_newline(&chars[*index], &chars[index.saturating_add(1)]);
            if is_newline {
                *skip_newlines = skip_newlines.saturating_add(skip);
                in_comment = false;
            }
            *index = index.saturating_add(skip);
            continue;
        }
        let (is_newline, skip) = is_newline(&chars[*index], &chars[index.saturating_add(1)]);
        if is_newline {
            *skip_newlines = skip_newlines.saturating_add(skip);
            *index = index.saturating_add(skip);
            continue;
        }
        let (is_whitespace, skip) =
            is_toml_whitespace(&chars[*index], &chars[index.saturating_add(1)]);
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
fn is_toml_whitespace(s: &str, next_char: &str) -> (bool, usize) {
    if s == "\t" || s == " " {
        if next_char == "\t" || next_char == " " {
            (true, 2)
        } else {
            (true, 1)
        }
    } else {
        is_newline(s, next_char)
    }
}

/// Matches unix & windows styles; for TOML 1.1.0 compliance
///
/// Also supports double `\n` to skip empty lines immediately
fn is_newline(s: &str, next_char: &str) -> (bool, usize) {
    if (s == "\r" || s == "\n") && next_char == "\n" {
        (true, 2)
    } else if s == "\n" {
        (true, 1)
    } else {
        (false, 0)
    }
}
