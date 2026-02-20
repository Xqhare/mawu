use std::{char, collections::{HashMap, VecDeque}};
use athena::XffValue;

use crate::{
    errors::{
        csv_error::{CsvError, CsvParseError},
        MawuError,
    },
    mawu_value::MawuValue,
    utils::is_newline,
};

pub fn headed(file_contents: VecDeque<char>) -> Result<MawuValue, MawuError> {
    let (head, left_content) = make_head(file_contents)?;
    let body = parse_csv_body(left_content, head.len())?;
    let mut out: Vec<HashMap<String, XffValue>> = Default::default();
    for entry in body {
        let mut tmp_bind: HashMap<String, XffValue> = Default::default();
        if entry.len() == head.len() {
            for (index, value) in entry.iter().enumerate() {
                tmp_bind.insert(head[index].clone(), value.clone());
            }
        } else {
            return Err(MawuError::CsvError(CsvError::ParseError(
                CsvParseError::ExtraValue(format!("{:?}", entry)),
            )));
        };
        out.push(tmp_bind);
    }
    Ok(MawuValue::CSVObject(out))
}

pub fn headless(file_contents: VecDeque<char>) -> Result<MawuValue, MawuError> {
    let (head, left_content) = make_head(file_contents)?;
    let mut body = parse_csv_body(left_content, head.len())?;
    body.insert(
        0,
        head.into_iter()
            .map(|s| XffValue::from(s))
            .collect::<Vec<XffValue>>(),
    );
    Ok(MawuValue::CSVArray(body))
}

fn parse_csv_body(
    mut csv_body: VecDeque<char>,
    head_length: usize,
) -> Result<Vec<Vec<XffValue>>, MawuError> {
    let mut out: Vec<Vec<XffValue>> = Default::default();
    let mut row_data: Vec<String> = Default::default();
    let mut last_char = None;
    while csv_body.front().is_some() {
        if let Some(h) = csv_body.pop_front() {
            if h == '\n' && csv_body.is_empty() {
                out.push(row_data.iter().map(|s| XffValue::from(s.clone())).collect());
                row_data = Default::default();
                break;
            }
            let is_next_newline: bool = {
                if let Some(k) = csv_body.front() {
                    if is_newline(k) {
                        true
                    } else {
                        false
                    }
                } else {
                    // no new character => end of file!
                    true
                }
            };
            if is_newline(&h) {
                if last_char.is_none() && head_length > row_data.len() || last_char.unwrap() == ',' && head_length > row_data.len() {
                    for _ in 0..(head_length - row_data.len()) {
                        row_data.push(String::from(""));
                    }
                } else if head_length > row_data.len() {
                    for _ in 0..(head_length - row_data.len()) {
                        row_data.push(String::from(""));
                    }
                }
                if is_next_newline {
                    let _ = csv_body.pop_front();
                }
                out.push(row_data.iter().map(|s| XffValue::from(s.clone())).collect());
                // assignment is only overwritten before being read if the very first character IS a newline and thus, probably, maybe, fine.
                row_data = Default::default();
            }  else if h == ',' {
                if is_next_newline && head_length > row_data.len() {
                    // push as many nulls as needed to fill in the missing data
                    for _ in 0..(head_length - row_data.len()) {
                        row_data.push(String::from(""));
                    }
                } else if last_char.is_none() || last_char.unwrap() == ',' {
                    row_data.push(String::from(""));
                }
            } else if h == '\"' {
                let mut value: String = Default::default();
                let mut open_quote = true;
                while open_quote {
                    if csv_body.front() == Some(&'\"') && csv_body.get(1) == Some(&'\"') {
                        value.push('\"');
                        let _ = csv_body.pop_front();
                        let _ = csv_body.pop_front();
                    } else if csv_body.front() == Some(&'\"') {
                        let _ = csv_body.pop_front();
                        open_quote = false;
                    } else {
                        if let Some(t) = csv_body.pop_front() {
                            value.push(t);
                        }
                    }
                }
                row_data.push(value);
            } else if h == ' ' || h == '\t' {
                let _ = h;
            } else {
                let mut value: String = h.to_string();
                while csv_body.front() != Some(&',')
                    && !is_newline(csv_body.front().unwrap_or(&'\n'))
                {
                    if let Some(t) = csv_body.pop_front() {
                        let mut entry = t.to_string();
                        while csv_body.front() != Some(&',')
                            && !is_newline(csv_body.front().unwrap_or(&'\n'))
                        {
                            if let Some(g) = csv_body.pop_front() {
                                entry.push(g);
                            }
                        }
                        entry = entry.trim_end().to_string();
                        value.push_str(&entry);
                    }
                }
                row_data.push(value);
            }
            last_char = Some(h)
        }
    }
    if !row_data.is_empty() {
        out.push(row_data.iter().map(|s| XffValue::from(s.clone())).collect());
    }
    Ok(out)
}

fn make_head(
    mut file_contents: VecDeque<char>,
) -> Result<(Vec<String>, VecDeque<char>), MawuError> {
    let mut head_done = false;
    let mut head_out: Vec<String> = Default::default();
    while !head_done {
        if let Some(content) = file_contents.pop_front() {
            if is_newline(&content) {
                head_done = true;
            } else if content == ',' {
                // do literally nothing
                let _ = content;
                continue;
            } else if content == ' ' || content == '\t' {
                // do literally nothing
                let _ = content;
                continue;
            } else {
                if content == '\"' {
                    let mut value: String = Default::default();
                    let mut open_quote = true;
                    while open_quote {
                        if file_contents.front() == Some(&'\"')
                            && file_contents.get(1) == Some(&'\"')
                        {
                            value.push('\"');
                            let _ = file_contents.pop_front();
                            let _ = file_contents.pop_front();
                        } else if file_contents.front() == Some(&'\"') {
                            let _ = file_contents.pop_front();
                            open_quote = false;
                        } else {
                            if let Some(t) = file_contents.pop_front() {
                                value.push(t);
                            }
                        }
                    }
                    head_out.push(value);
                } else {
                    let mut value: String = content.to_string();
                    while file_contents.front() != Some(&',')
                        && !is_newline(file_contents.front().ok_or_else(|| {
                            MawuError::CsvError(CsvError::ParseError(
                                CsvParseError::UnexpectedNewline,
                            ))
                        })?)
                    {
                        if let Some(t) = file_contents.pop_front() {
                            let mut entry = t.to_string();
                            while file_contents.front() != Some(&',')
                                && !is_newline(file_contents.front().ok_or_else(|| {
                                    MawuError::CsvError(CsvError::ParseError(
                                        CsvParseError::UnrecognizedHeader("".to_string()),
                                    ))
                                })?)
                            {
                                if let Some(g) = file_contents.pop_front() {
                                    entry.push(g);
                                }
                            }
                            let entry = entry.trim_end().to_string();
                            value.push_str(&entry);
                        }
                    }
                    head_out.push(value);
                }
            }
        } else {
            let t = file_contents
                .iter()
                .map(|s| format!("{}", s))
                .collect::<String>();
            return Err(MawuError::CsvError(CsvError::ParseError(
                CsvParseError::UnrecognizedHeader(t),
            )));
        };
    }
    if file_contents.front() == Some(&'\n') {
        let _ = file_contents.pop_front();
    }
    Ok((head_out, file_contents))
}
