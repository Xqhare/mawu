use std::char::{self, decode_utf16};

use crate::errors::{MawuError, MawuInternalError};

pub mod file_handling;

/// Takes in a `usize` and returns a `String` that is `n` spaces long filled with whitespace
pub fn make_whitespace<N: Into<usize> + Copy>(n: N) -> String {
    let mut s: String = Default::default();
    // upcasting is ok
    while s.len() < n.into() {
        s = " ".to_string() + &s;
    }
    s
}

/// Takes in a `&str` and checks if it is a newline character
/// (either `\n` or `\r`)
///
/// ## Returns
/// `true` if the string is a newline, `false` otherwise
pub fn is_newline(s: &char) -> bool {
    s == &'\n' || s == &'\r'
}

/// Takes in two `&str` and unescapes unicode characters
///
/// ## Arguments
/// * `s` - The string to unescape
/// * `next_codepoint` - The next codepoint to unescape in the case of a surrogate pair,
///   can be left empty if the string is guaranteed to be not a surrogate pair
///
/// ## Returns
/// `Ok((String, bool))` if the string is successfully unescaped, `Err(MawuError)` otherwise
/// the boolean is `true` if the next_codepoint was used, `false` otherwise
pub fn unescape_unicode(s: &str, next_codepoint: &str) -> Result<(String, bool), MawuError> {
    if let Ok(out) = my_unescape_unicode_handler(s.to_string()) {
        Ok((out, false))
    } else if next_codepoint.is_empty() {
        Err(MawuError::InternalError(
            MawuInternalError::UnableToUnescapeUnicode(s.to_string()),
        ))
    } else {
        let tmp: Vec<u16> = vec![
            u16::from_str_radix(s, 16).unwrap(),
            u16::from_str_radix(next_codepoint, 16).unwrap(),
        ];
        let out = decode_utf16(tmp.iter().copied()).next().unwrap();
        if let Ok(c) = out {
            Ok((c.to_string(), true))
        } else {
            Err(MawuError::InternalError(
                MawuInternalError::UnableToUnescapeUnicode(s.to_string()),
            ))
        }
    }
}

fn my_unescape_unicode_handler(s: String) -> Result<String, MawuError> {
    let mut unicode_value = 0u32;
    for char in s.chars() {
        if let Some(d) = char.to_digit(0x10) {
            // Just a casual bit-shift and a bitwise OR to build the unicode value
            // I can Invoke the ancient inscriptions, and think I know what they are doing:
            // I have a value of 32 0s, and each char is 4 bits long.
            // To make space for the next char, I just shift the current value by 4 along, and
            // then, as the shifted value is all 0s again, I just copy the char into it with a bitwise OR (0,0 == 0, 0,1 == 1)
            unicode_value = (unicode_value << 4) + d;
            // Check if the unicode value is above 0x10FFFF (the maximum value of a unicode codepoint)
            if unicode_value > 0x10FFFF {
                return Err(MawuError::InternalError(
                    MawuInternalError::UnableToUnescapeUnicode(s.to_string()),
                ));
            }
        } else {
            // If the character is not a digit, it is an error!
            return Err(MawuError::InternalError(
                MawuInternalError::UnableToUnescapeUnicode(s.to_string()),
            ));
        }
    }
    let possible_char = char::from_u32(unicode_value);
    // user supplied data always needs to be checked, invalid data can always be supplied
    if let Some(c) = possible_char {
        Ok(c.to_string())
    } else {
        Err(MawuError::InternalError(
            MawuInternalError::UnableToUnescapeUnicode(s.to_string()),
        ))
    }
}

/// Takes in a `&str` and checks the very first character to see if it is a digit
///
/// ## Returns
/// `true` if the first character is a digit, `false` otherwise
///
/// ## Errors
/// `MawuError::InternalError` if the string has no characters
pub fn is_digit(c: &char) -> Result<bool, MawuError> {
    // This if loop has proven to be faster than the char method `is_digit` by a very slight
    // margin. But it is faster! (Using `match` is slower than both `if` and `char` methods)
    Ok(('0'..='9').contains(c))
}

pub fn is_end_of_primitive_value(c: char) -> bool {
    c == ',' || c == ':' || c == '}' || c == ']'
}

pub fn is_whitespace(c: &char) -> bool {
    is_newline(c) || c == &' ' || c == &'\t'
}

/// Returns true if the given character is a json string terminator (':','}',']')
/// Do not forget to check for end of file!
/// Uses `\n` as end of file making it compatible with modern windows, linux and some OSX versions.
pub fn is_json_string_terminator_token(c: Option<&char>) -> bool {
    if c.is_none() {
        return false;
    }
    let c = c.unwrap();
    *c == ':' || *c == ',' || *c == '}' || *c == ']'
}

use athena::XffValue;
/// Automatically converts a string into an XffValue (Number, Boolean, Null or String)
pub fn xff_from_string_auto(s: String) -> XffValue {
    if s.is_empty() {
        XffValue::Null
    } else if let Ok(u) = s.parse::<u64>() {
        XffValue::from(u as usize)
    } else if let Ok(i) = s.parse::<i64>() {
        XffValue::from(i as isize)
    } else if let Ok(f) = s.parse::<f64>() {
        if f.is_nan() || f.is_infinite() {
            XffValue::Null
        } else {
            XffValue::from(f)
        }
    } else if let Ok(b) = s.parse::<bool>() {
        XffValue::from(b)
    } else {
        XffValue::from(s)
    }
}
