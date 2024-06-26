use archiver::ArchiverOpts;
use clap::Parser;
use cli::{CliOpts, Mode, PackmanError, Result};
use extractor::{create_extractor, ExtractorOpts};

mod archiver;
mod cli;
mod extractor;
mod format;

fn execute(opts: &mut CliOpts) -> Result<()> {
    match opts.run_mode() {
        Ok(Mode::Archive) => return execute_archive(&opts),
        Ok(Mode::Extract) => return execute_extract(&opts),
        _ => return Ok(()),
    }
}

fn execute_archive(opts: &CliOpts) -> Result<()> {
    let archiver_opts = ArchiverOpts::new(&opts);
    if let Err(e) = archiver_opts.exit_paths() {
        return Err(e);
    }
    match archiver::create_archiver(&opts.output.clone().unwrap()) {
        Ok(archiver) => archiver.execute(archiver_opts),
        Err(e) => Err(e),
    }
}

fn execute_extract(opts: &CliOpts) -> Result<()> {
    let extractor_opts = ExtractorOpts::new(&opts);
    for target in &opts.args {
        match create_extractor(target) {
            Ok(extractor) => extractor.execute(target.clone(), &extractor_opts)?,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut opts = CliOpts::parse();
    match execute(&mut opts) {
        Ok(_) => Ok(()),
        Err(e) => {
            match e {
                PackmanError::NoArgumentsGiven => {
                    println!("No arguments given. Use --help for usage.")
                }
                PackmanError::ArgumentsPathNotFound => {
                    println!("One or more arguments are not valid paths.")
                }
                PackmanError::FileExists(p) => {
                    println!("{}: file already exists", p.to_str().unwrap())
                }
                PackmanError::IOError(e) => println!("IO error: {}", e),
                PackmanError::ArchiverError(s) => println!("Archive error: {}", s),
                PackmanError::ExtractorError(s) => println!("Extractor error: {}", s),
            }
            std::process::exit(1);
        }
    }
}
