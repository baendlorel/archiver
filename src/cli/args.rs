use clap::Parser;

use super::archiver_command::ArchiverCommand;

#[derive(Parser)]
#[command(name="Archiver", author=env!("CARGO_PKG_AUTHORS"), version=concat!("v", env!("CARGO_PKG_VERSION"), " by ", env!("CARGO_PKG_AUTHORS")), about="ReadMe: https://github.com/baendlorel/archiver/blob/main/README.md", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<ArchiverCommand>,
}
