// the 'unused_imports' warning is a false positive, they are needed for the tests
#![allow(unused_imports)]
use std::{
    char, collections::{HashMap, VecDeque}, rc::Rc, sync::{Mutex, MutexGuard}
};

use athena::{XffValue, Number, Array, Object};

use crate::{
    errors::{
        json_error::{JsonError, JsonParseError},
        MawuError, MawuInternalError,
    },
    utils::{
        file_handling::read_file, is_digit, is_end_of_primitive_value, is_json_string_terminator_token, is_whitespace, unescape_unicode
    },
};

pub fn json_lexer(file_contents: VecDeque<char>) -> Result<XffValue, MawuError> {
    if !file_contents.is_empty() {
        let contents_store: Rc<Mutex<VecDeque<char>>> = Rc::new(Mutex::new(file_contents));
        let res = if let Ok(mut contents) = contents_store.try_lock() {
            json_value_lexer(&mut contents)
        } else {
            Err(MawuError::InternalError(
                MawuInternalError::UnableToLockMasterMutex,
            ))
        };
        res
    } else {
        Ok(XffValue::default())
    }
}

fn json_value_lexer(
    file_contents: &mut MutexGuard<VecDeque<char>>,
) -> Result<XffValue, MawuError> {
    while file_contents.front().is_some() {
        let this_char = file_contents.pop_front().unwrap();
        // Ignore whitespace
        // As formatted JSON files contain a lot of whitespace leave this as the first check
        // as it's more efficient and doesn't matter otherwise
        if is_whitespace(&this_char) {
            continue;
        }
        // Actual parsing
        if this_char == '{' {
            // object
            return json_object_lexer(file_contents);
        } else if this_char == '[' {
            // array
            return json_array_lexer(file_contents);
        } else if this_char == 'N' && file_contents.front() == Some(&'a') && file_contents.get(1) == Some(&'N') || this_char == 'n' && file_contents.front() == Some(&'a') && file_contents.get(1) == Some(&'n') {
            // NaN
            return Err(MawuError::JsonError(JsonError::ParseError(
                JsonParseError::InvalidNumber("NaN".to_string()),
            )));
        } else if this_char == 'I' && file_contents.front() == Some(&'n') && file_contents.get(1) == Some(&'f') || this_char == 'i' && file_contents.front() == Some(&'n') && file_contents.get(1) == Some(&'f') {
            // Infinity
            return Err(MawuError::JsonError(JsonError::ParseError(
                JsonParseError::InvalidNumber("Infinity".to_string()),
            )));
        } else if this_char == 't'
            && file_contents.front() == Some(&'r')
            && file_contents.get(1) == Some(&'u')
            && file_contents.get(2) == Some(&'e')
        {
            // true
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            return Ok(XffValue::Boolean(true));
        } else if this_char == 'f'
            && file_contents.front() == Some(&'a')
            && file_contents.get(1) == Some(&'l')
            && file_contents.get(2) == Some(&'s')
            && file_contents.get(3) == Some(&'e')
        {
            // false
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            return Ok(XffValue::Boolean(false));
        } else if this_char == 'n'
            && file_contents.front() == Some(&'u')
            && file_contents.get(1) == Some(&'l')
            && file_contents.get(2) == Some(&'l')
        {
            // null
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            return Ok(XffValue::Null);
        } else if this_char == '}' || this_char == ']' || this_char == ',' || this_char == ':' {
            // Invalid json grammar
            return Err(MawuError::JsonError(JsonError::ParseError(
                JsonParseError::InvalidStructuralToken(this_char.to_string()),
            )));
        } else if this_char == '\"' {
            // string
            return json_string_lexer(file_contents);
        } else if this_char == '-' || is_digit(&this_char)? {
            // number
            return json_number_lexer(
                file_contents,
                if this_char != '-' {
                    Some(this_char)
                } else {
                    None
                },
            );
        } else {
            // Invalid json grammar
            return Err(MawuError::JsonError(JsonError::ParseError(
                JsonParseError::InvalidCharacter(this_char.to_string()),
            )));
        }
    }
    Err(MawuError::JsonError(JsonError::ParseError(
        JsonParseError::UnexpectedEndOfFile,
    )))
}

fn json_object_lexer(
    file_contents: &mut MutexGuard<VecDeque<char>>,
) -> Result<XffValue, MawuError> {
    let mut binding_object: Object = Default::default();
    while file_contents.front() != Some(&'}') && file_contents.front().is_some() {
        if is_whitespace(file_contents.front().unwrap()) {
            let _ = file_contents.pop_front();
            continue;
        }
        if file_contents.front() == Some(&',') {
            let _ = file_contents.pop_front();
            continue;
        }
        if file_contents.front() == Some(&'\n') && file_contents.len() <= 1 {
            let _ = file_contents.pop_front();
            return Ok(XffValue::Object(binding_object));
        }
        let key = json_value_lexer(file_contents)?.to_string();
        if file_contents.front() == Some(&':') {
            let _ = file_contents.pop_front();
            let value = json_value_lexer(file_contents)?;
            binding_object.insert(key, value);
        } else {
            return Err(MawuError::JsonError(JsonError::ParseError(
                JsonParseError::ExpectedColon,
            )));
        }
    }
    if file_contents.front() == Some(&'}') {
        let _ = file_contents.pop_front();
        Ok(XffValue::Object(binding_object))
    } else {
        Err(MawuError::JsonError(JsonError::ParseError(
            JsonParseError::ExpectedEndOfObject,
        )))
    }
}

fn json_array_lexer(
    file_contents: &mut MutexGuard<VecDeque<char>>,
) -> Result<XffValue, MawuError> {
    let mut binding_array: Array = Default::default();
    while file_contents.front() != Some(&']') && file_contents.front().is_some() {
        if is_whitespace(file_contents.front().unwrap()) {
            let _ = file_contents.pop_front();
            continue;
        }
        if file_contents.front() == Some(&',') {
            let _ = file_contents.pop_front();
            continue;
        }
        if file_contents.front() == Some(&'\n') && file_contents.len() <= 1 {
            let _ = file_contents.pop_front();
            return Ok(XffValue::Array(binding_array));
        }
        let value = json_value_lexer(file_contents)?;
        binding_array.push(value);
    }
    if file_contents.front() == Some(&']') {
        let _ = file_contents.pop_front();
    }
    Ok(XffValue::Array(binding_array))
}

fn json_string_lexer(
    file_contents: &mut MutexGuard<VecDeque<char>>,
) -> Result<XffValue, MawuError> {
    let mut string: String = Default::default();
    loop {
        if let Some(character) = file_contents.pop_front() {
            let next_char = file_contents.front();
            // End of string
            // Or part checks for end of file
            if character == '\"' && is_json_string_terminator_token(next_char)
                || file_contents.is_empty()
                || file_contents.front() == Some(&'\n') && file_contents.len() <= 1
            {
                return Ok(XffValue::String(string));
            }
            // the two nested if statements are joined, meaning that only if `\"` is encountered
            // AND the next char is whitespace the logic is executed
            if character == '\"' && next_char.is_some() {
                let next_char = file_contents.pop_front().unwrap();
                if is_whitespace(&next_char) {
                    while is_whitespace(file_contents.front().unwrap()) {
                        let _ = file_contents.pop_front().unwrap();
                    }

                    if is_json_string_terminator_token(file_contents.front()) {
                        return Ok(XffValue::String(string));
                    } else {
                        return Err(MawuError::JsonError(JsonError::ParseError(
                            JsonParseError::UnexpectedCharacter(
                                file_contents.front().unwrap().to_string(),
                            ),
                        )));
                    }
                }
            }
            // Escape character
            // remember, some characters have two chars of escape sequence (`\"` being represented
            // as ["\\", "\""])
            else if character == '\\' {
                if next_char.is_some() {
                    let next_char = file_contents.pop_front().unwrap();
                    if next_char == 'u' {
                        // after a u there can only ever be 4 hex-digits
                        if file_contents.len() >= 4 {
                            let hex1 = file_contents.pop_front().unwrap();
                            let hex2 = file_contents.pop_front().unwrap();
                            let hex3 = file_contents.pop_front().unwrap();
                            let hex4 = file_contents.pop_front().unwrap();
                            let next_codepoint = {
                                if file_contents.len() >= 6 {
                                    let mut out: String = Default::default();
                                    out.push(*file_contents.get(2).unwrap());
                                    out.push(*file_contents.get(3).unwrap());
                                    out.push(*file_contents.get(4).unwrap());
                                    out.push(*file_contents.get(5).unwrap());
                                    out
                                } else {
                                    String::default()
                                }
                            };
                            let tmp = unescape_unicode(
                                &format!("{}{}{}{}", hex1, hex2, hex3, hex4),
                                &next_codepoint,
                            );
                            if let Ok((out, codepointused)) = tmp {
                                // next codepoint was used
                                // so we pop it off, including the skipped `\u`
                                if codepointused {
                                    let _ = file_contents.pop_front();
                                    let _ = file_contents.pop_front();
                                    let _ = file_contents.pop_front();
                                    let _ = file_contents.pop_front();
                                    let _ = file_contents.pop_front();
                                    let _ = file_contents.pop_front();
                                }
                                string.push_str(&out);
                            } else {
                                return Err(MawuError::JsonError(JsonError::ParseError(
                                    JsonParseError::InvalidEscapeSequence(format!(
                                        "{}{}",
                                        character, next_char
                                    )),
                                )));
                            }
                            continue;
                        }
                    } else if next_char == '/' {
                        string.push('/');
                    } else if next_char == 'b' {
                        string.push('\u{0008}');
                    } else if next_char == 'f' {
                        string.push('\u{000C}');
                    } else if next_char == 'n' {
                        string.push('\n');
                    } else if next_char == 'r' {
                        string.push('\r');
                    } else if next_char == 't' {
                        string.push('\t');
                    } else if next_char == '\\' {
                        string.push('\\');
                    } else if next_char == '\"' {
                        string.push('"');
                    } else {
                        Err(MawuError::JsonError(JsonError::ParseError(
                            JsonParseError::InvalidEscapeSequence(format!(
                                "{}{}",
                                character, next_char
                            )),
                        )))?
                    }
                } else {
                    Err(MawuError::JsonError(JsonError::ParseError(
                        JsonParseError::UnexpectedEndOfFile,
                    )))?
                }
            // Only space is accepted as whitespace in json, the rest has to be escaped
            } else if character == ' ' {
                string.push(' ');
            } else if character == '\"' {
                return Err(MawuError::JsonError(JsonError::ParseError(
                    JsonParseError::InvalidEscapeSequence(format!("{}", character)),
                )));
            } else {
                string.push(character);
            }
        }
    }
}

fn json_number_lexer(
    file_contents: &mut MutexGuard<VecDeque<char>>,
    first_digit: Option<char>,
) -> Result<XffValue, MawuError> {
    let mut out: String = Default::default();
    if let Some(digit) = first_digit {
        out.push(digit);
    } else {
        out.push('-');
    }
    while !file_contents.is_empty() {
        let this_char = file_contents.pop_front().unwrap();
        if is_whitespace(&this_char) {
            continue;
        }
        if this_char == '.' || is_digit(&this_char)? {
            out.push(this_char);
        } else if this_char == 'e' || this_char == 'E' {
            out.push(this_char);
            if file_contents.front() == Some(&'+') || file_contents.front() == Some(&'-') {
                out.push(file_contents.pop_front().unwrap());
            } else if is_digit(file_contents.front().unwrap())? {
                out.push('+');
            } else {
                return Err(MawuError::JsonError(JsonError::ParseError(
                    JsonParseError::InvalidCharacter(this_char.to_string()),
                )));
            }
        } else if is_end_of_primitive_value(this_char) {
            file_contents.push_front(this_char);
            break;
        } else {
            return Err(MawuError::JsonError(JsonError::ParseError(
                JsonParseError::InvalidCharacter(this_char.to_string()),
            )));
        }
    }
    
    // Parse the number string into XffValue
    if out.contains('.') || out.contains('e') || out.contains('E') {
        if let Ok(f) = out.parse::<f64>() {
            if f.is_infinite() || f.is_nan() {
                return Ok(XffValue::Null);
            }
            return Ok(XffValue::Number(Number::Float(f)));
        }
    } else if let Ok(u) = out.parse::<u64>() {
        return Ok(XffValue::Number(Number::Unsigned(u as usize)));
    } else if let Ok(i) = out.parse::<i64>() {
        return Ok(XffValue::Number(Number::Integer(i as isize)));
    }

    Err(MawuError::JsonError(JsonError::ParseError(
        JsonParseError::InvalidNumber(out),
    )))
}

#[test]
fn object_lexer() {
    let input = json_lexer(
        read_file("data/json/json-test-data/rfc8259-test-data/object.json").unwrap()
    );
    assert!(input.is_ok());
}

#[test]
fn array_lexer() {
    let input = json_lexer(
        read_file("data/json/json-test-data/rfc8259-test-data/array.json").unwrap()
    );
    assert!(input.is_ok());
}

#[test]
fn string_lexer() {
    let double_quotes = vec!['\"', '\\', '\"', '\"'];
    let parsed_quotes = json_lexer(double_quotes.clone().into());
    assert!(parsed_quotes.is_ok());
    assert!(parsed_quotes.unwrap() == XffValue::String("\"".to_string()));

    let unicode = vec![
        '\"', '\\', 'u', '2', '6', '0', '3', '\\', 'u', '0', '0', '2', '6', '\\', 'u', '0', '0',
        'E', 'A', '\\', 'u', 'F', 'B', '2', '0', '\"',
    ];
    let parsed_unicode = json_lexer(unicode.into());
    assert!(parsed_unicode.is_ok());
    assert!(
        parsed_unicode.unwrap()
            == XffValue::String("\u{2603}\u{0026}\u{00EA}\u{FB20}".to_string())
    );

    let backslash = vec!['\"', '\\', '\\', '\"'];
    let parsed_backslash = json_lexer(backslash.into());
    assert!(parsed_backslash.is_ok());
    assert!(parsed_backslash.unwrap() == XffValue::String("\\".to_string()));

    let slash = vec!['\"', '\\', '/', '\"'];
    let parsed_slash = json_lexer(slash.into());
    assert!(parsed_slash.is_ok());
    assert!(parsed_slash.unwrap() == XffValue::String("/".to_string()));

    let mut tmp = "\"backspace\"".to_string();
    tmp.insert_str(4, r"\b");
    let backspace = tmp.chars().collect::<VecDeque<char>>();
    let parsed_backspace = json_lexer(backspace);
    assert!(parsed_backspace.is_ok());
    assert!(parsed_backspace.unwrap().into_string().unwrap() == "bac\u{0008}kspace");

    let formfeed = vec!['\"', '\\', 'f', 'f', '\"'];
    let parsed_formfeed = json_lexer(formfeed.into());
    assert!(parsed_formfeed.is_ok());
    assert!(parsed_formfeed.unwrap() == XffValue::String("\u{000C}f".to_string()));

    let newline = vec!['\"', '\\', 'n', 'n', '\"'];
    let parsed_newline = json_lexer(newline.into());
    assert!(parsed_newline.is_ok());
    assert!(parsed_newline.unwrap() == XffValue::String("\nn".to_string()));

    let return_test = vec!['\"', '\\', 'r', 'r', '\"'];
    let parsed_return = json_lexer(return_test.into());
    assert!(parsed_return.is_ok());
    assert!(parsed_return.unwrap() == XffValue::String("\rr".to_string()));

    let tab = vec!['\"', '\\', 't', ' ', 't', 'e', 's', 't', '\"'];
    let parsed_tab = json_lexer(tab.into());
    assert!(parsed_tab.is_ok());
    assert!(parsed_tab.unwrap() == XffValue::String("\t test".to_string()));
}

#[test]
fn number_lexer() {
    let small_neg = VecDeque::from(vec!['-', '1', '2', '3']);
    let small_neg_res = json_lexer(small_neg).unwrap();
    assert_eq!(small_neg_res, XffValue::from(-123));
    let small_pos = VecDeque::from(vec!['1', '2', '3']);
    let small_pos_res = json_lexer(small_pos).unwrap();
    assert_eq!(small_pos_res, XffValue::from(123usize));

    let large_neg = VecDeque::from(vec!['-', '9', '8', '7', '6', '5', '4', '3', '2', '1']);
    let large_neg_res = json_lexer(large_neg).unwrap();
    assert_eq!(large_neg_res, XffValue::from(-987654321isize));
    let large_pos = VecDeque::from(vec!['9', '8', '7', '6', '5', '4', '3', '2', '1']);
    let large_pos_res = json_lexer(large_pos).unwrap();
    assert_eq!(large_pos_res, XffValue::from(987654321usize));

    let small_float = VecDeque::from(vec!['1', '.', '2', '3']);
    let easy_float_res = json_lexer(small_float).unwrap();
    assert_eq!(easy_float_res, XffValue::from(1.23));
    let small_neg_float = VecDeque::from(vec!['-', '1', '.', '2', '3']);
    let small_neg_float_res = json_lexer(small_neg_float).unwrap();
    assert_eq!(small_neg_float_res, XffValue::from(-1.23));

    let large_float = VecDeque::from(vec!['9', '.', '8', '7', '6', '5', '4', '3', '2', '1']);
    let large_float_res = json_lexer(large_float).unwrap();
    assert_eq!(large_float_res, XffValue::from(9.87654321));
    let large_neg_float =
        VecDeque::from(vec!['-', '9', '.', '8', '7', '6', '5', '4', '3', '2', '1']);
    let large_neg_float_res = json_lexer(large_neg_float).unwrap();
    assert_eq!(large_neg_float_res, XffValue::from(-9.87654321));

    let small_exp = VecDeque::from(vec!['1', '.', '2', '3', 'e', '+', '1', '2']);
    let small_exp_res = json_lexer(small_exp).unwrap();
    assert_eq!(small_exp_res, XffValue::from(1230000000000.0));
    let small_neg_exp = VecDeque::from(vec!['-', '1', '.', '2', '3', 'e', '+', '1', '2']);
    let small_neg_exp_res = json_lexer(small_neg_exp).unwrap();
    assert_eq!(small_neg_exp_res, XffValue::from(-1230000000000.0));

    let large_exp = VecDeque::from(vec![
        '9', '.', '8', '7', '6', '5', '4', '3', '2', '1', 'e', '+', '1', '2',
    ]);
    let large_exp_res = json_lexer(large_exp).unwrap();
    assert_eq!(large_exp_res, XffValue::from(9876543210000.0));
    let large_neg_exp = VecDeque::from(vec![
        '-', '9', '.', '8', '7', '6', '5', '4', '3', '2', '1', 'e', '+', '1', '2',
    ]);
    let large_neg_exp_res = json_lexer(large_neg_exp).unwrap();
    assert_eq!(large_neg_exp_res, XffValue::from(-9876543210000.0));

    let neg_exp = VecDeque::from(vec!['1', '.', '2', '3', 'e', '-', '9']);
    let neg_exp_res = json_lexer(neg_exp).unwrap();
    assert_eq!(neg_exp_res, XffValue::from(0.00000000123));
    let neg_exp2 = VecDeque::from(vec!['-', '1', '.', '2', '3', 'e', '-', '1', '2']);
    let neg_exp2_res = json_lexer(neg_exp2).unwrap();
    assert_eq!(neg_exp2_res, XffValue::from(-0.00000000000123));

    let small_exp_float_no_plus_after_e = VecDeque::from(vec!['1', '.', '2', '3', 'e', '1', '2']);
    let small_exp_float_res = json_lexer(small_exp_float_no_plus_after_e).unwrap();
    assert_eq!(small_exp_float_res, XffValue::from(1230000000000.0));
    let small_neg_exp_float_no_plus_after_e =
        VecDeque::from(vec!['-', '1', '.', '2', '3', 'e', '1', '2']);
    let small_neg_exp_float_res = json_lexer(small_neg_exp_float_no_plus_after_e).unwrap();
    assert_eq!(small_neg_exp_float_res, XffValue::from(-1230000000000.0));
}
