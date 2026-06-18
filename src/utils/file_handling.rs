use std::{collections::VecDeque, fs::read_to_string, path::Path};
use nemesis::NemesisError;

/// This function reads the contents of a file, and converts the bytes from a Vec<u8> to a `VecDeque`<char>.
/// It only accepts valid UTF-8 encoded files, returning an error otherwise.
pub fn read_file<T: AsRef<Path>>(path: T) -> Result<VecDeque<char>, NemesisError> {
    let path_ref = path.as_ref();
    let out = read_to_string(path_ref)
        .map_err(|e| {
            NemesisError::new("mawu::utils::file_handling", e)
                .add_ctx(format!("Failed to read file at: {}", path_ref.display()))
        })?
        .chars()
        .collect::<VecDeque<char>>();
    Ok(out)
}

/// This function writes a file with the given contents.
pub fn write_file<T: AsRef<Path>, C: AsRef<[u8]>>(path: T, contents: C) -> Result<(), NemesisError> {
    let path_ref = path.as_ref();
    std::fs::write(path_ref, contents).map_err(|e| {
        NemesisError::new("mawu::utils::file_handling", e)
            .add_ctx(format!("Failed to write file at: {}", path_ref.display()))
    })
}
