use std::path::PathBuf;

use crate::cli::{PackmanError, Result};

#[derive(Debug, PartialEq)]
pub enum Format {
    SevenZip,
    Zip,
}

pub fn find_format(dest: &PathBuf) -> Result<Format> {
    let name = dest.to_str().unwrap().to_lowercase();
    if name.ends_with(".7z") {
        Ok(Format::SevenZip)
    } else if name.ends_with(".zip") {
        Ok(Format::Zip)
    } else {
        Err(PackmanError::NoArgumentsGiven)
    }
}
