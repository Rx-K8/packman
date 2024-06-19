use crate::cli::{PackmanError, Result};
use std::ffi::OsStr;

pub fn find_format(file_name: Option<&OsStr>) -> Result<Format> {
    match file_name {
        Some(file_name) => {
            let name = file_name.to_str().unwrap().to_lowercase();
            if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
                return Ok(Format::TarGz);
            } else if name.ends_with(".tar.bz2") || name.ends_with(".tbz2") {
                return Ok(Format::TarBz2);
            } else if name.ends_with(".tar.xz") || name.ends_with(".txz") {
                return Ok(Format::TarXz);
            } else if name.ends_with(".tar.zst") || name.ends_with(".tzst") {
                return Ok(Format::TarZstd);
            } else if name.ends_with(".7z") {
                return Ok(Format::SevenZ);
            } else if name.ends_with(".tar") {
                return Ok(Format::Tar);
            } else if name.ends_with(".lha") || name.ends_with(".lzh") {
                return Ok(Format::LHA);
            } else if name.ends_with(".rar") {
                return Ok(Format::Rar);
            } else if name.ends_with(".zip")
                || name.ends_with(".jar")
                || name.ends_with(".war")
                || name.ends_with(".ear")
            {
                return Ok(Format::Zip);
            } else {
                return Ok(Format::Unknown(file_name.to_str().unwrap().to_string()));
            }
        }
        None => Err(PackmanError::NoArgumentsGiven),
    }
}

#[derive(Debug, PartialEq)]
pub enum Format {
    Zip,
    Tar,
    TarGz,
    TarBz2,
    TarXz,
    TarZstd,
    SevenZ,
    LHA,
    Rar,
    Unknown(String),
}
