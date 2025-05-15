use clap::{CommandFactory, Parser};
use owo_colors::OwoColorize;

mod cli;
mod handlers;
mod misc;
mod models;

use cli::{Args, ArvCmd};

fn main() {
    // 启用堆栈跟踪
    let args = Args::parse();

    match args.command {
        Some(ArvCmd::Archive { target }) => handlers::archive::handler(target),
        Some(ArvCmd::Restore { id }) => handlers::restore::handler(id),
        Some(ArvCmd::List { all }) => handlers::list::handler(all),
        Some(ArvCmd::Log { range }) => handlers::log::handler(range),
        Some(ArvCmd::Config {
            alias,
            alias_list,
            alias_remove,
        }) => {
            // 和上面range不一样，这里的选项参数是必须要写的，所以先判定后调用handler
            if let Some(alias_entry) = alias {
                handlers::config::handler_alias(alias_entry);
                return;
            }
            if alias_list {
                handlers::config::handler_alias_list();
                return;
            }
            if let Some(alias_entry) = alias_remove {
                handlers::config::handler_alias_remove(alias_entry);
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
