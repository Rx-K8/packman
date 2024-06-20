use std::fs::{read_dir, File};
use std::path::PathBuf;

use crate::archiver::{Archiver, ArchiverOpts};
use crate::cli::{PackmanError, Result};
use crate::format::Format;
use sevenz_rust::{SevenZArchiveEntry, SevenZWriter};

pub(super) struct SevenZipArchiver {}

impl Archiver for SevenZipArchiver {
    fn execute(&self, opts: ArchiverOpts) -> Result<()> {
        match opts.destination() {
            Ok(dest) => write_sevenzip(dest, opts.targets.clone(), opts.recursive),
            Err(e) => Err(e),
        }
    }

    fn format(&self) -> Format {
        Format::SevenZip
    }
}

fn write_sevenzip(destination: File, targets: Vec<PathBuf>, recursive: bool) -> Result<()> {
    let mut zw = SevenZWriter::new(destination).unwrap();
    for target in targets {
        if target.is_dir() && recursive {
            process_dir(&mut zw, target)?
        } else {
            process_file(&mut zw, target)?
        }
    }
    Ok(())
}

fn process_file(sz: &mut SevenZWriter<File>, target: PathBuf) -> Result<()> {
    let name = target.to_str().unwrap();
    if let Err(e) = sz.push_archive_entry(
        SevenZArchiveEntry::from_path(&target, name.to_string()),
        Some(File::open(target).unwrap()),
    ) {
        return Err(PackmanError::ArchiverError(e.to_string()));
    }
    Ok(())
}

fn process_dir(sz: &mut SevenZWriter<File>, target: PathBuf) -> Result<()> {
    for entry in read_dir(target).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                process_dir(sz, path)?;
            } else {
                process_file(sz, path)?;
            }
        }
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
    fn test_format() {
        let archiver = SevenZipArchiver {};
        assert_eq!(archiver.format(), Format::SevenZip);
    }

    #[test]
    fn test_sevenzip() {
        run_test(|| {
            let archiver = SevenZipArchiver {};
            let opts = ArchiverOpts {
                dest: PathBuf::from("results/test.7z"),
                targets: vec![PathBuf::from("src"), PathBuf::from("Cargo.toml")],
                recursive: true,
                overwrite: true,
            };
            let result = archiver.execute(opts);
            assert!(result.is_ok());
        });
    }

    fn teardown() {
        let _ = std::fs::remove_file("results/test.7z");
    }
}
