use crate::{handlers::log, models::types::OperType};

mod alias;
mod auto_check_update;
pub mod config_data;
mod list;

// todo 改造这里，命令格式化为arv config alias.add xxx
// alias.remove  xxx | auto-xxx=on |
pub fn handler(
    config_item: &Option<Option<String>>,
    alias: &Option<String>,
    alias_remove: &Option<String>,
    auto_check_update: &Option<String>,
) {
    // println!("config_item: {:?}", config_item);
    // println!("alias: {:?}", alias);
    // println!("alias_remove: {:?}", alias_remove);
    // println!("auto_check_update: {:?}", auto_check_update);

    if let Some(config_item) = config_item {
        // 进入这里说明输入了--list，下面判定是否给了后续参数
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

fn handle_show(config_item: &Option<String>) {
    if let Err(e) = list::show(config_item) {
        println!("Show aliases failed. {}", e.to_string());
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
            println!("Auto check update is set to '{}'.", arg);
            log::succ(&oper, arg, None, None);
        }
        Err(e) => log::err(&oper, arg, None, e),
    }
}
