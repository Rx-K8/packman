use std::io::Read;
use std::io::{BufReader, Seek, Write};
use std::path::PathBuf;
use zip::write::SimpleFileOptions;

use crate::cli::Result;
use std::fs::File;
use zip::write::ZipWriter;

use crate::archiver::{Archiver, ArchiverOpts, Format};
use crate::cli::PackmanError;

pub(super) struct ZipArchiver {

}

impl Archiver for ZipArchiver {
    fn execute(&self, archiver_opts: &ArchiverOpts) -> Result<()> {
        match archiver_opts.destination() {
            Err(e) => Err(e),
            Ok(file) => write_zip(file, archiver_opts.targets(), archiver_opts.recursive),
        }
    }

    fn format(&self) -> Format {
        Format::Zip
    }
}

fn write_zip(file: File, targets: Vec<PathBuf>, recursive: bool) -> Result<()> {
    let mut zw = ZipWriter::new(file);
    for target in targets {
        if target.is_dir() &&  recursive{
            process_dir(&mut zw, target.clone())?;
        } else {
            process_file(&mut zw, target.clone())?;
        }
    }
    if let Err(e) = zw.finish() {
        return Err(PackmanError::ArchiverError(e.to_string()));
    }
    Ok(())
}

fn process_dir<W: Write + Seek>(zw: &mut ZipWriter<W>, target: PathBuf) -> Result<()> {
    for entry in target.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            process_dir(zw, path)?;
        } else {
            process_file(zw, path)?;
        }
    }
    Ok(())
}

fn create_file(target: &PathBuf) -> Result<File> {
    match File::open(&target) {
        Ok(file) => Ok(file),
        Err(e) => Err(PackmanError::IOError(e)),
    }
}

fn process_file<W: Write + Seek>(zw: &mut ZipWriter<W>, target: PathBuf) -> Result<()> {
    let mut reader = BufReader::new(create_file(&target)?);
    let mut contents = String::new();
    if let Err(e) = reader.read_to_string(&mut contents) {
        return Err(PackmanError::IOError(e));
    }
    let options = SimpleFileOptions::default();
    let file_name = target.to_str().unwrap();
    if let Err(e) = zw.start_file(file_name, options) {
        return Err(PackmanError::ArchiverError(e.to_string()));
    }
    if let Err(e) = zw.write(contents.as_bytes()) {
        return Err(PackmanError::IOError(e));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test<F>(f: F)
    where
        F: FnOnce(),
    {
        // setup(); // 予めやりたい処理
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
        teardown(); // 後片付け処理

        if let Err(err) = result {
            std::panic::resume_unwind(err);
        }
    }

    #[test]
    fn test_zip() {
        run_test(|| {
            let archiver = ZipArchiver {};
            let inout = ArchiverOpts::create(
                PathBuf::from("results/test.zip"),
                vec![PathBuf::from("src"), PathBuf::from("Cargo.toml")],
                true,
                true,
            );
            let result = archiver.execute(&inout);
            assert!(result.is_ok());
            assert_eq!(archiver.format(), Format::Zip);
        });
    }

    fn teardown() {
        let _ = std::fs::remove_file("results/test.zip");
    }
}
