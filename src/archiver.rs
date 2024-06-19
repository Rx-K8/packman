use zip::ZipArchiver;

use crate::cli::{CliOpts, PackmanError, Result};
use crate::format::{find_format, Format};
use std::fs::{create_dir_all, File};
use std::path::PathBuf;

mod zip;

pub trait Archiver {
    fn execute(&self, archive_opts: &ArchiverOpts) -> Result<()>;
    fn format(&self) -> Format;
}

pub fn create_archiver(dest: &PathBuf) -> Result<Box<dyn Archiver>> {
    let format = find_format(dest.file_name());
    match format {
        Ok(format) => Ok(Box::new(ZipArchiver {})),

        Err(msg) => Err(msg),
    }
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

    pub fn create(
        output: PathBuf,
        targets: Vec<PathBuf>,
        recursive: bool,
        overwrite: bool,
    ) -> Self {
        ArchiverOpts {
            output,
            targets,
            recursive,
            overwrite,
        }
    }

    pub fn targets(&self) -> Vec<PathBuf> {
        self.targets.clone()
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
