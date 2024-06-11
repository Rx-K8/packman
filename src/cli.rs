use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliOpts {
    #[clap(short = 'm', long="mode", default_value_t = Mode::Auto, value_name = "MODE", required = false, ignore_case = true,  value_enum, help = "Mode of operation.")]
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
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Copy)]
pub enum Mode {
    Auto,
    Archive,
    Extract,
    List,
}
