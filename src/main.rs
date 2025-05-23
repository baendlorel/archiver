use clap::{CommandFactory, Parser};
use owo_colors::OwoColorize;

mod cli;
mod handlers;
mod misc;
mod models;

use cli::{Args, ArvCmd};

fn main() {
    let args: Args = Args::parse();
    apply_command(&args);
    auto_check_update(&args);
}

fn apply_command(args: &Args) {
    match &args.command {
        Some(ArvCmd::Put { targets }) => handlers::put::handler(&targets),
        Some(ArvCmd::Restore { ids }) => handlers::restore::handler(&ids),
        Some(ArvCmd::List { all, restored }) => handlers::list::handler(*all, *restored),
        Some(ArvCmd::Log { range }) => handlers::log::handler(range),
        Some(ArvCmd::Config { statement }) => handlers::config::handler(&statement),
        Some(ArvCmd::Update) => handlers::update::handler(),
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
        Some(ArvCmd::Update) => false,
        Some(ArvCmd::List {
            all: _,
            restored: _,
        }) => false,
        Some(ArvCmd::Log { range: _ }) => false,
        None => false,
        _ => true,
    };

    if need_checking {
        handlers::update::auto_check();
    }
}
