use std::{collections::VecDeque, fs::read_to_string, path::Path};

use crate::errors::MawuError;

/// This function reads the contents of a file, and converts the bytes from a Vec<u8> to a VecDeque<char>.
/// It only accepts valid UTF-8 encoded files, returning an error otherwise.
pub fn read_file<T: AsRef<Path>>(path: T) -> Result<VecDeque<char>, MawuError> {
    let out = read_to_string(path.as_ref()).map_err(MawuError::IoError)?.chars().collect::<VecDeque<char>>();
    Ok(out)
}

/// This function writes a file with the given contents.
pub fn write_file<T: AsRef<Path>, C: AsRef<[u8]>>(path: T, contents: C) -> Result<(), MawuError> {
    std::fs::write(path.as_ref(), contents).map_err(MawuError::IoError)
}
