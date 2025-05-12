use clap::{CommandFactory, Parser};
use owo_colors::OwoColorize;

mod cli;
mod handlers;
mod misc;
mod models;

use cli::{Args, ArvCmd};

fn main() {
    let args = Args::parse();

    match args.command {
        Some(ArvCmd::List) => handlers::list::handler(),
        Some(ArvCmd::Log { interval }) => handlers::log::handler(interval),
        Some(ArvCmd::Restore { id }) => handlers::restore::handler(id),
        Some(ArvCmd::Archive { target }) => handlers::archive::handler(target),
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
