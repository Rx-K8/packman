use clap::{Parser, ValueEnum};
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, PackmanError>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliOpts {
    #[clap(
        short = 'm',
        long="mode",
        default_value_t = Mode::Auto, 
        value_name = "MODE",
        required = false,
        ignore_case = true,
        value_enum,
        help = "Mode of operation."
    )]
    pub mode: Mode,
    #[clap(
        short = 'o',
        long = "output",
        value_name = "OUTPUT",
        required = false,
        help = "Output file in archive mode, or output directory in extraction mode"
    )]
    pub output: Option<PathBuf>,
    #[clap(
        short = 'r',
        long = "recursive",
        default_value_t = true,
        help = "Recurse into directories (archive mode)."
    )]
    pub recursive: bool,
    #[clap(
        long = "overwrite",
        default_value_t = false,
        required = false,
        help = "Overwrite existing files."
    )]
    pub overwrite: bool,
    #[clap(
        value_name = "ARGUMENTS",
        help = "List of files or directories to be processed."
    )]
    pub args: Vec<PathBuf>,
}

impl CliOpts {
    pub fn run_mode(&mut self) -> Result<Mode> {
        if self.args.len() == 0 {
            return Err(PackmanError::NoGivenArguments);
        }

        if self.mode == Mode::Auto {
            if is_extract_mode(&self.args) {
                self.mode = Mode::Extract;
                return Ok(Mode::Extract);
            } else {
                self.mode = Mode::Archive;
                return Ok(Mode::Archive);
            }
        } else {
            return Ok(self.mode);
        }
    }
}

fn is_extract_mode(args: &Vec<PathBuf>) -> bool {
    let archive_exts = vec![".zip", ".tar", ".tar.gz", ".tgz", ".tar.bz2", ".tbz2", ".rar", ".jar", ".war", ".ear", ".7z", ];
    args.iter().all(|arg| {
        let name = arg.to_str().unwrap().to_lowercase();
        for archive_ext in archive_exts.iter() {
            if name.ends_with(archive_ext) {
                return true;
            }
        }
        return false;
    })
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Copy)]
pub enum Mode {
    Auto,
    Archive,
    Extract,
    List,
}

#[derive(Debug)]
pub enum PackmanError {
    NoGivenArguments,
    FileExists(PathBuf),
    IOError(std::io::Error),
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use clap::Parser;

    use super::*;

    #[test]
    fn test_find_mode() {
        let mut cli1 = CliOpts::parse_from(&["totebag_test", "src", "LICENSE", "README.md", "Cargo.toml"]);
        let r1 = cli1.run_mode();
        assert!(r1.is_ok());
        assert_eq!(r1.unwrap(), Mode::Archive);

        let mut cli2 = CliOpts::parse_from(&["totebag_test", "src", "LICENSE", "README.md", "hoge.zip"]);
        let r2 = cli2.run_mode();
        assert!(r2.is_ok());
        assert_eq!(cli2.run_mode().unwrap(), Mode::Archive);

        let mut cli3 = CliOpts::parse_from(&["totebag_test", "src.zip", "LICENSE.tar", "README.tar.bz2", "hoge.rar"]);
        let r3 = cli3.run_mode();
        assert!(r3.is_ok());
        assert_eq!(cli3.run_mode().unwrap(), Mode::Extract);

        let mut cli4 = CliOpts::parse_from(&["totebag_test", "src.zip", "LICENSE.tar", "README.tar.bz2", "hoge.rar", "--mode", "list"]);
        let r4 = cli3.run_mode();
        assert!(r4.is_ok());
        assert_eq!(cli4.run_mode().unwrap(), Mode::List);
    }

        #[test]
    fn test_is_extract_mode() {
        assert!(is_extract_mode(&vec![PathBuf::from("test.zip"), PathBuf::from("test.tar"), PathBuf::from("test.tar.gz"), PathBuf::from("test.tgz"), PathBuf::from("test.tar.bz2"), PathBuf::from("test.tbz2"), PathBuf::from("test.rar")]));
    }
}
