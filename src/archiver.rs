use crate::cli::CliOpts;
use crate::cli::PackmanError;
use crate::cli::Result;
use crate::format::Format;
use std::fs::create_dir_all;
use std::fs::File;
use std::path::PathBuf;

mod sevenzip;
mod zip;

pub trait Archiver {
    fn execute(&self, opts: &ArchiverOpts) -> Result<()>;
    fn format(&self) -> Format;
}

pub struct ArchiverOpts {
    pub dest: PathBuf,
    pub targets: Vec<PathBuf>,
    pub recursive: bool,
    pub overwrite: bool,
}

impl ArchiverOpts {
    pub fn new(opts: &CliOpts) -> Self {
        let args = opts.args.clone();
        let dest = opts.output.clone().unwrap_or(PathBuf::from("."));
        ArchiverOpts {
            dest: dest,
            targets: args,
            recursive: opts.recursive,
            overwrite: opts.overwrite,
        }
    }

    pub fn destination(&self) -> Result<File> {
        if self.dest.is_file() & self.dest.exists() & !self.overwrite {
            return Err(PackmanError::FileExists(self.dest.clone()));
        }
        if let Some(parent) = self.dest.parent() {
            if !parent.exists() {
                if let Err(e) = create_dir_all(parent) {
                    return Err(PackmanError::IOError(e));
                }
            }
        }
        match File::create(&self.dest) {
            Ok(file) => Ok(file),
            Err(e) => Err(PackmanError::IOError(e)),
        }
    }
}
