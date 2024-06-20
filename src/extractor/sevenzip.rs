use crate::cli::{PackmanError, Result};
use crate::extractor::{Extractor, ExtractorOpts};
use crate::format::Format;
use std::fs::File;
use std::path::PathBuf;

pub(super) struct SevenZipExtractor {}

impl Extractor for SevenZipExtractor {
    fn execute(&self, target: PathBuf, opts: &ExtractorOpts) -> Result<()> {
        let reader = match File::open(&target) {
            Ok(f) => f,
            Err(e) => return Err(PackmanError::IOError(e)),
        };

        if let Err(e) = sevenz_rust::decompress(reader, opts.dest.clone()) {
            return Err(PackmanError::ExtractorError(e.to_string()));
        };
        Ok(())
    }
    fn format(&self) -> Format {
        Format::SevenZip
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_list() {
    //     let extractor = SevenZExtractor{};
    //     let file = PathBuf::from("testdata/test.7z");
    //     match extractor.list_archives(file) {
    //         Ok(r) => {
    //             assert_eq!(r.len(), 21);
    //             assert_eq!(r.get(0), Some("Cargo.toml".to_string()).as_ref());
    //             assert_eq!(r.get(1), Some("build.rs".to_string()).as_ref());
    //             assert_eq!(r.get(2), Some("LICENSE".to_string()).as_ref());
    //             assert_eq!(r.get(3), Some("README.md".to_string()).as_ref());
    //         },
    //         Err(_) => assert!(false),
    //     }
    // }

    #[test]
    fn test_extract_archive() {
        let e = SevenZipExtractor {};
        let file = PathBuf::from("testdata/test.7z");
        let opts = ExtractorOpts {
            dest: PathBuf::from("results/sevenz"),
            overwrite: true,
        };
        match e.execute(file, &opts) {
            Ok(_) => {
                assert!(true);
                assert!(PathBuf::from("results/sevenz/Cargo.toml").exists());
                std::fs::remove_dir_all(PathBuf::from("results/sevenz")).unwrap();
            }
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn test_format() {
        let e = SevenZipExtractor {};
        assert_eq!(e.format(), Format::SevenZip);
    }
}
