use std::path::PathBuf;

use crate::cli::{CliOpts, Result};
use crate::format::{find_format, Format};

mod sevenzip;
mod zip;

pub struct ExtractorOpts {
    pub dest: PathBuf,
    pub overwrite: bool,
}

impl ExtractorOpts {
    pub fn new(opts: &CliOpts) -> Self {
        let dest = opts.output.clone().unwrap_or_else(|| PathBuf::from("."));
        ExtractorOpts {
            dest: dest.clone(),
            overwrite: opts.overwrite,
        }
    }
}

pub trait Extractor {
    fn execute(&self, target: PathBuf, opts: &ExtractorOpts) -> Result<()>;
    fn format(&self) -> Format;
}

pub fn create_extractor(target: &PathBuf) -> Result<Box<dyn Extractor>> {
    match find_format(target) {
        Ok(Format::SevenZip) => Ok(Box::new(sevenzip::SevenZipExtractor {})),
        Ok(Format::Zip) => Ok(Box::new(zip::ZipExtractor {})),
        Err(e) => Err(e),
    }
}
