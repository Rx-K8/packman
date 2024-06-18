use std::io::Read;
use std::io::{BufReader, Seek, Write};
use std::path::PathBuf;
use zip::write::SimpleFileOptions;

use std::fs::File;
use zip::result::ZipResult;
use zip::write::ZipWriter;

use crate::archiver::{Archiver, ArchiverOpts, Format};

pub(super) struct ZipArchiver {}

impl Archiver for ZipArchiver {
    fn execute(&self, archiver_opts: &ArchiverOpts) -> Result<()> {
        match archiver_opts.destination() {
            Err(e) => Err(e),
            Ok(file) => write_zip(archiver_opts),
        }
    }
    fn format(&self) -> Format {
        Format::Zip
    }
}

fn write_zip(archiver_opts: &ArchiverOpts) -> Result<()> {
    let mut zw = ZipWriter::new(File::create(&archiver_opts.output)?);
    for target in &archiver_opts.targets {
        if target.is_dir() {
            process_dir(&mut zw, target.clone())?;
        } else {
            process_file(&mut zw, target.clone())?;
        }
    }
    zw.finish()?;
    Ok(())
}

fn process_dir<W: Write + Seek>(zw: &mut ZipWriter<W>, target: PathBuf) -> Result<()> {
    for entry in target.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            process_dir(zw, path)?;
        } else {
            process_file(zw, path)?;
        }
    }
    Ok(())
}

fn process_file<W: Write + Seek>(zw: &mut ZipWriter<W>, target: PathBuf) -> Result<()> {
    let mut reader = BufReader::new(File::open(&target).unwrap());
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let options = SimpleFileOptions::default();
    let file_name = target.to_str().unwrap();
    zw.start_file(file_name, options)?;
    zw.write(contents.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_file() -> ZipResult<()> {
        let target = PathBuf::from("src/main.rs");
        let mut zw = ZipWriter::new(File::create("test1.zip")?);
        let result = process_file(&mut zw, target);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_process_file2() -> ZipResult<()> {
        let target =
            PathBuf::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.txt");
        let mut zw = ZipWriter::new(File::create("test2.zip")?);
        let result = process_file(&mut zw, target);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_process_dir() -> ZipResult<()> {
        let target = PathBuf::from("src");
        let mut zw = ZipWriter::new(File::create("test3.zip")?);
        let result = process_dir(&mut zw, target);
        assert!(result.is_ok());
        Ok(())
    }
}
