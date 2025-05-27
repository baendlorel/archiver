mod cli;
mod core;
mod handlers;
mod misc;
mod models;

use cli::{ArchiverCommand as AC, FULL_CMD};

fn main() {
    handle(&FULL_CMD);
    auto_check_update(&FULL_CMD);
}

fn handle(command: &AC) {
    match command {
        AC::Put {
            targets,
            message,
            vault,
        } => handlers::put(&targets, message, vault),
        AC::Restore { ids } => handlers::restore(&ids),
        AC::Vault(action) => handlers::vault(&action),
        AC::Move { ids, to } => handlers::mv(ids, to),
        AC::List { all, restored } => handlers::list(*all, *restored),
        AC::Log { range } => handlers::log(range),
        AC::Config(action) => handlers::config(&action),
        AC::Update => handlers::update(),
    }
}

fn auto_check_update(command: &AC) {
    let need_checking = match command {
        AC::Update => false,
        AC::List {
            all: _,
            restored: _,
        } => false,
        AC::Log { range: _ } => false,
        _ => true,
    };

    if need_checking {
        core::update::auto_check();
    }
}
