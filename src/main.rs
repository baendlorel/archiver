use clap::{CommandFactory, Parser};
use owo_colors::OwoColorize;

mod cli;
mod commands;
mod misc;
mod models;

use cli::{ArchiverCommand, Args};

fn main() {
    let args = Args::parse();

    match args.command {
        Some(ArchiverCommand::List { name }) => {
            commands::list::handler(name);
        }
        Some(ArchiverCommand::Log { time_interval }) => {
            commands::log::handler(time_interval);
        }
        Some(ArchiverCommand::Restore { target }) => {
            commands::restore::handler(target);
        }
        Some(ArchiverCommand::Archive { target }) => {
            commands::archive::handler(target);
        }
        None => {
            println!("{}", "请指定一个操作命令".yellow());
            // 打印帮助信息
            // 一定要顶部写use clap::{CommandFactory, Parser};
            // 下边的Args::command()才能成立，否则会说：
            // * items from traits can only be used if the trait is in scoperustcClick for full compiler diagnostic
            Args::command().print_help().expect("无法打印帮助信息");
            println!(); // 添加一个空行
        }
    }
}
