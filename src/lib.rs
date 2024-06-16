pub mod errors;
mod lexers;
pub mod mawu_values;
mod utils;

pub mod csv {
    use std::{collections::VecDeque, path::Path};
    use unicode_segmentation::UnicodeSegmentation;

    use crate::{
        errors::MawuError, lexers::csv_lexer, mawu_values::MawuValue, utils::file_handling,
    };

    pub fn read_csv_headed<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        csv_lexer::headed(
            file_handling::read_file(path)?
                .graphemes(true)
                .collect::<VecDeque<&str>>(),
        )
    }

    pub fn read_csv_headless<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        csv_lexer::headless(
            file_handling::read_file(path)?
                .graphemes(true)
                .collect::<VecDeque<&str>>(),
        )
    }
}
