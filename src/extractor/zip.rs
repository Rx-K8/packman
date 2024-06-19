use crate::cli::Result;
use crate::extractor::{Extractor, ExtractorOpts};
use crate::format::Format;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::PathBuf;

pub struct ZipExtractor {}

impl Extractor for ZipExtractor {
    fn list_archives(&self, archive_file: PathBuf) -> Result<Vec<String>> {
        let zip_file = File::open(archive_file).unwrap();
        let mut zip = zip::ZipArchive::new(zip_file).unwrap();

        let mut result = Vec::<String>::new();

        for i in 0..zip.len() {
            let file = zip.by_index(i).unwrap();
            result.push(file.name().to_string());
        }
        Ok(result)
    }

    fn execute(&self, archive_file: PathBuf, output: PathBuf) -> Result<()> {
        let mut zip = zip::ZipArchive::new(File::open(archive_file).unwrap()).unwrap();
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            if file.is_file() {
                let file_path = output.join(PathBuf::from(file.name().to_string()));
                create_dir_all(output.clone()).unwrap();
                let mut out = File::create(file_path.clone()).unwrap();
                copy(&mut file, &mut out).unwrap();
            }
        }
        Ok(())
    }
    fn format(&self) -> Format {
        Format::Zip
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_archive() {
        let extractor = ZipExtractor {};
        let file = PathBuf::from("testdata/test.zip");
        let opts = ExtractorOpts {
            output: PathBuf::from("results/zip"),
            overwrite: true,
        };
        match extractor.execute(file, opts.output.clone()) {
            Ok(_) => {
                assert!(true);
                assert!(PathBuf::from("results/zip/Cargo.toml").exists());
                std::fs::remove_dir_all(PathBuf::from("results/zip")).unwrap();
            }
            Err(_) => {
                assert_eq!(true, false);
            }
        }
    }
}
