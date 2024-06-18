use crate::cli::{CliOpts, Result, PackmanError};
use crate::format::Format;
use ::zip::result::ZipResult;
use std::path::PathBuf;
use std::fs::{create_dir_all, File};

mod zip;

pub trait Archiver {
    fn execute(&self, archive_opts: &ArchiverOpts) -> ZipResult<()>;
    fn format(&self) -> Format;
}

pub struct ArchiverOpts {
    pub output: PathBuf,
    pub targets: Vec<PathBuf>,
    pub recursive: bool,
    pub overwrite: bool,
}

impl ArchiverOpts {
    pub fn new(opts: &CliOpts) -> Self {
        let args = opts.args.clone();
        let output = opts.output.clone().unwrap_or_else(|| PathBuf::from("."));
        Self {
            output: output,
            targets: args,
            recursive: opts.overwrite,
            overwrite: opts.recursive,
        }
    }

    pub fn destination(&self) -> Result<File> {
        let p = self.output.as_path();
        if p.exists() && p.is_file() && !self.overwrite {
            return Err(PackmanError::FileExists(self.output.clone()));
        }
        if let Some(parent) = p.parent() {
            if !parent.exists() {
                if let Err(e) = create_dir_all(parent) {
                    return Err(PackmanError::IOError(e));
                }
            }
        }
        match File::create(&self.output) {
            Ok(f) => Ok(f),
            Err(e) => Err(PackmanError::IOError(e)),
        }
    }
}
