use owo_colors::OwoColorize;

use crate::{handlers::log, misc::paths, models::types::OperType};

mod alias;
mod auto_check_update;
mod config_data;
mod list;

pub fn handler(
    config_item: &Option<String>,
    alias: &Option<String>,
    alias_remove: &Option<String>,
    auto_check_update: &Option<String>,
) {
    // todo copilot 貌似说的不对：
    // 用户输入 arv config --list 或 arv config --list alias，config_item 是 Some(...)。用户没有输入 --list，config_item 就是 None。
    if let Some(config_item) = config_item {
        handle_show(config_item);
    } else if let Some(alias) = alias {
        handle_alias(alias);
    } else if let Some(alias_remove) = alias_remove {
        handle_alias_remove(alias_remove);
    } else if let Some(auto_check_update) = auto_check_update {
        handle_auto_check_update(auto_check_update);
    }
}

fn handle_alias(arg: &str) {
    let oper = OperType::Config {
        option: "--alias".to_string(),
    };
    match alias::set_alias(&arg) {
        Ok(_) => {
            println!("Alias '{}' is set successfully.", arg);
            log::succ(&oper, arg, None, None);
        }
        Err(e) => log::err(&oper, arg, None, e),
    }
}

fn handle_show(config_item: &str) {
    match config_data::load() {
        Ok(config) => {
            println!(
                "Alias entries:\n  ~={} {}",
                paths::HOME_DIR.to_string_lossy().to_string(),
                "(default)".cyan()
            );
            for entry in config.alias_list {
                let content = format!("{}={}", entry.alias, entry.origin,);
                println!("  {}", content);
            }
        }
        Err(e) => println!("Show aliases failed. {}", e.to_string()),
    }
}

fn handle_alias_remove(arg: &str) {
    let oper = OperType::Config {
        option: "--alias-remove".to_string(),
    };

    match alias::remove_alias(&arg) {
        Ok(_) => {
            println!("Alias '{}' is removed successfully.", arg);
            log::succ(&oper, arg, None, None);
        }
        Err(e) => log::err(&oper, arg, None, e),
    }
}

fn handle_auto_check_update(arg: &str) {
    let oper = OperType::Config {
        option: "--auto-check-update".to_string(),
    };

    match auto_check_update::toggle(&arg) {
        Ok(_) => {
            println!("Alias '{}' is removed successfully.", arg);
            log::succ(&oper, arg, None, None);
        }
        Err(e) => log::err(&oper, arg, None, e),
    }
}
