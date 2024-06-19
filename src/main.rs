use archiver::ArchiverOpts;
use cli::{CliOpts, Mode, PackmanError};

use crate::cli::Result;
use clap::Parser;

mod archiver;
mod cli;
mod extractor;
mod format;

fn execute(mut opts: CliOpts) -> Result<()> {
    match opts.run_mode() {
        Ok(Mode::Archive) => return execute_archive(opts),
        Ok(Mode::Extract) => return execute_extract(opts),
        Ok(Mode::List) => return execute_list(opts),
        Ok(Mode::Auto) => {
            return Err(PackmanError::UnknownError(
                "cannot distinguish archiving and extracting".to_string(),
            ))
        }
        Err(e) => {
            return Err(e);
        }
    };
}

fn execute_extract(opts: CliOpts) -> Result<()> {
    let args = opts.args.clone();
    for arg in args.iter() {
        let extractor = extractor::create_extractor(arg).unwrap();
        let target = arg.to_path_buf();
        extractor.execute(target, opts.output.clone().unwrap())?;
    }
    Ok(())
}

fn execute_archive(opts: CliOpts) -> Result<()> {
    let archiver_opts = ArchiverOpts::new(&opts);
    match archiver::create_archiver(&opts.output.unwrap()) {
        Ok(archiver) => archiver.execute(&archiver_opts),
        Err(e) => Err(e),
    }
}

fn execute_list(opts: CliOpts) -> Result<()> {
    let args = opts.args.clone();
    for arg in args.iter() {
        if !arg.exists() {
            return Err(PackmanError::FileNotFound(arg.to_path_buf()));
        }
        let extractor = extractor::create_extractor(&arg).unwrap();
        if args.len() > 1 {
            println!("========== {:?} ========== \n", arg);
        }
        let files = extractor.list_archives(arg.to_path_buf()).unwrap();
        for file in files.iter() {
            println!("{}", file);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    match execute(CliOpts::parse()) {
        Ok(_) => Ok(()),
        Err(e) => {
            match e {
                PackmanError::NoArgumentsGiven => {
                    println!("No arguments given. Use --help for usage.")
                }
                PackmanError::FileNotFound(p) => {
                    println!("{}: file not found", p.to_str().unwrap())
                }
                PackmanError::FileExists(p) => {
                    println!("{}: file already exists", p.to_str().unwrap())
                }
                PackmanError::IOError(e) => println!("IO error: {}", e),
                PackmanError::ArchiverError(s) => println!("Archive error: {}", s),
                PackmanError::SomeError(e) => println!("Error: {}", e),
                PackmanError::UnknownError(s) => println!("Unknown error: {}", s),
            }
            std::process::exit(1);
        }
    }
}
