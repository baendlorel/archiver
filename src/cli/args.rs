use clap::{CommandFactory, Parser};
use once_cell::sync::Lazy;
use owo_colors::OwoColorize;

use super::archiver_command::ArchiverCommand;

#[derive(Parser)]
#[command(name="Archiver", author=env!("CARGO_PKG_AUTHORS"), version=concat!("v", env!("CARGO_PKG_VERSION"), " by ", env!("CARGO_PKG_AUTHORS"),"\nTry `arv put <path>` to archive your objects now!"), about="ReadMe: https://github.com/baendlorel/archiver/blob/main/README.md", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<ArchiverCommand>,
}

pub static FULL_CMD: Lazy<ArchiverCommand> = Lazy::new(|| {
    if let Some(command) = Args::parse().command {
        command
    } else {
        println!("{}", "Please enter your command".yellow());
        // 打印帮助信息
        // 一定要顶部写use clap::{CommandFactory, Parser};
        // 下边的Args::command()才能成立，否则会说：
        Args::command()
            .print_help()
            .expect("Cannot print help text");
        println!(); // 添加一个空行
        std::process::exit(1)
    }
});
