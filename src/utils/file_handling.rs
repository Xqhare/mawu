use nemesis::NemesisError;
use std::{
    collections::VecDeque,
    fs::{read, read_to_string},
    path::Path,
};
use thoth::Thoth;

/// This function reads the contents of a file, and converts the bytes from a Vec<u8> to a `VecDeque`<char>.
/// It only accepts valid UTF-8 encoded files, returning an error otherwise.
pub fn read_file<T: AsRef<Path>>(path: T) -> Result<VecDeque<char>, NemesisError> {
    Ok(read_to_string(path.as_ref())
        .map_err(|e| {
            NemesisError::new("mawu::utils::file_handling", e).add_ctx(format!(
                "Failed to read file at: {}",
                path.as_ref().display()
            ))
        })?
        .chars()
        .collect::<VecDeque<char>>())
}

/// This function reads the contents of a file, and converts the bytes from a Vec<u8> to a `Vec<char>`.
/// It only accepts valid UTF-8 encoded files, returning an error otherwise.
pub fn read_file_unicode_segment<T: AsRef<Path>>(path: T) -> Result<Vec<String>, NemesisError> {
    Thoth::grapheme_segmentation_u8(&read(path.as_ref()).map_err(|e| {
        NemesisError::new("mawu::utils::file_handling", e).add_ctx(format!(
            "Failed to read file at: {}",
            path.as_ref().display()
        ))
    })?)
}

/// This function writes a file with the given contents.
pub fn write_file<T: AsRef<Path>, C: AsRef<[u8]>>(
    path: T,
    contents: C,
) -> Result<(), NemesisError> {
    std::fs::write(path.as_ref(), contents).map_err(|e| {
        NemesisError::new("mawu::utils::file_handling", e).add_ctx(format!(
            "Failed to write file at: {}",
            path.as_ref().display()
        ))
    })
}
