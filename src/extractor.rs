use crate::format::find_format;
use crate::{
    cli::{CliOpts, PackmanError, Result},
    format::Format,
};
use std::path::PathBuf;

pub mod zip;

pub struct ExtractorOpts {
    pub output: PathBuf,
    pub overwrite: bool,
}

impl ExtractorOpts {
    pub fn new(opts: CliOpts) -> ExtractorOpts {
        let output = opts.output.clone().unwrap_or_else(|| PathBuf::from("."));
        ExtractorOpts {
            output: output,
            overwrite: opts.recursive,
        }
    }
}

pub trait Extractor {
    fn list_archives(&self, archive_file: PathBuf) -> Result<Vec<String>>;
    fn execute(&self, archive_file: PathBuf, output: PathBuf) -> Result<()>;
    fn format(&self) -> Format;
}

pub fn create_extractor(file: &PathBuf) -> Result<Box<dyn Extractor>> {
    let format = find_format(file.file_name());
    match format {
        Ok(format) => return Ok(Box::new(zip::ZipExtractor {})),
        Err(msg) => Err(msg),
    }
}
