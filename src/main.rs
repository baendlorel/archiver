use clap::{CommandFactory, Parser};
use owo_colors::OwoColorize;

mod cli;
mod core;
mod handlers;
mod misc;
mod models;

use cli::{ArchiverCommand as AC, Args};

fn main() {
    let args: Args = Args::parse();
    apply_command(&args);
    auto_check_update(&args);
}

fn apply_command(args: &Args) {
    match &args.command {
        Some(AC::Put { targets }) => handlers::put(&targets),
        Some(AC::Restore { ids }) => handlers::restore(&ids),
        Some(AC::List { all, restored }) => handlers::list(*all, *restored),
        Some(AC::Log { range }) => handlers::log(range),
        Some(AC::Config { statement }) => handlers::config(&statement),
        Some(AC::Update) => handlers::update(),
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

fn auto_check_update(args: &Args) {
    let need_checking = match &args.command {
        Some(AC::Update) => false,
        Some(AC::List {
            all: _,
            restored: _,
        }) => false,
        Some(AC::Log { range: _ }) => false,
        None => false,
        _ => true,
    };

    if need_checking {
        core::update::auto_check();
    }
}
