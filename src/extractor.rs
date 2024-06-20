use std::path::PathBuf;

use crate::cli::{CliOpts, Result};
use crate::format::Format;

mod zip;
mod sevenzip;

pub struct ExtractorOpts {
    pub dest: PathBuf,
    pub overwrite: bool,
}

impl ExtractorOpts {
    pub fn new(opts: CliOpts) -> Self {
        let dest = opts.output.unwrap_or_else(|| PathBuf::from("."));
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
