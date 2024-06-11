use clap::Parser;
use cli::CliOpts;

mod cli;

fn main() {
    let cli_opts = CliOpts::parse();
    println!("{:?}", cli_opts)
}
