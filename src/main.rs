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
        Some(ArvCmd::List { all }) => handlers::list::handler(all),
        Some(ArvCmd::Log { range }) => handlers::log::handler(range),
        Some(ArvCmd::Restore { id }) => handlers::restore::handler(id),
        Some(ArvCmd::Archive { target }) => handlers::archive::handler(target),
        Some(ArvCmd::Config { alias, alias_list }) => {
            handlers::config::handler_alias(alias);
            if alias_list {
                handlers::config::handler_alias_list();
            }
        }
        Some(ArvCmd::Clear) => {
            println!(
                "This is dangerous and we will not implement it. If you really want to clear the archive, just remove the '.archive' folder in your home dir."
            );
        }
        None => {
            println!("{}", "Please enter your command".yellow());
            // 打印帮助信息
            // 一定要顶部写use clap::{CommandFactory, Parser};
            // 下边的Args::command()才能成立，否则会说：
            // * items from traits can only be used if the trait is in scoperustcClick for full compiler diagnostic
            Args::command()
                .print_help()
                .expect("Cannot print help text");
            println!(); // 添加一个空行
        }
    }
}
