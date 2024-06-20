use super::{Extractor, ExtractorOpts};
use crate::cli::{PackmanError, Result};
use crate::format::Format;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::PathBuf;

pub(super) struct ZipExtractor {}

impl Extractor for ZipExtractor {
    fn execute(&self, target: PathBuf, opts: &ExtractorOpts) -> Result<()> {
        let reader = match File::open(&target) {
            Ok(f) => f,
            Err(e) => return Err(PackmanError::IOError(e)),
        };
        let mut zip = zip::ZipArchive::new(reader).unwrap();

        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            if file.is_file() {
                let destination = opts.dest.join(file.name());
                create_dir_all(destination.parent().unwrap()).unwrap();
                let mut out = File::create(destination.clone()).unwrap();
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
    use std::path::PathBuf;

    // #[test]
    // fn test_list_archives() {
    //     let extractor = ZipExtractor {};
    //     let file = PathBuf::from("testdata/test.zip");
    //     match extractor.list_archives(file) {
    //         Ok(r) => {
    //             assert_eq!(r.len(), 19);
    //             assert_eq!(r.get(0), Some("Cargo.toml".to_string()).as_ref());
    //             assert_eq!(r.get(1), Some("build.rs".to_string()).as_ref());
    //             assert_eq!(r.get(2), Some("LICENSE".to_string()).as_ref());
    //             assert_eq!(r.get(3), Some("README.md".to_string()).as_ref());
    //         }
    //         Err(_) => assert!(false),
    //     }
    // }

    #[test]
    fn test_extract_archive() {
        let e = ZipExtractor {};
        let file = PathBuf::from("testdata/test.zip");
        let opts = ExtractorOpts {
            dest: PathBuf::from("results/zip"),
            overwrite: true,
        };
        match e.execute(file, &opts) {
            Ok(_) => {
                assert!(true);
                assert!(PathBuf::from("results/zip/Cargo.toml").exists());
                std::fs::remove_dir_all(PathBuf::from("results/zip")).unwrap();
            }
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn test_format() {
        let e = ZipExtractor {};
        assert_eq!(e.format(), Format::Zip);
    }
}
