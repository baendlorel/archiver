mod cli;
mod core;
mod handlers;
mod misc;
mod models;
mod traits;

use cli::{ArchiverCommand as AC, FULL_CMD};

fn main() {
    // 这样传参会自动对Lazy进行deref，如果想直接match，那么需要手动deref
    handle(&FULL_CMD);
    auto_check_update(&FULL_CMD);
}

fn handle(command: &AC) {
    match command {
        AC::Put {
            items,
            message,
            vault,
        } => handlers::put(items, message, vault),
        AC::Restore { ids } => handlers::restore(&ids),
        AC::Vault(action) => handlers::vault(&action),
        AC::Move { ids, to } => handlers::mov(ids, to),
        AC::List { all, restored } => handlers::list(*all, *restored),
        AC::Log { range, id } => handlers::log(range, id),
        AC::Config(action) => handlers::config(&action),
        AC::Update => handlers::update(),
        AC::Check { verbose } => handlers::check(verbose),
    }
}

fn auto_check_update(command: &AC) {
    let need_checking = match command {
        AC::Update => false,
        AC::List { .. } => false,
        AC::Log { .. } => false,
        AC::Check { .. } => false,
        _ => true,
    };

    if need_checking {
        core::update::auto_check();
    }
}
