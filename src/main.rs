use clap::Parser;
use cli::CliOpts;

mod cli;

fn main() {
    let mut cli_opts = CliOpts::parse();
    let _mode = cli_opts.run_mode();
    println!("{:?}", cli_opts)
}
