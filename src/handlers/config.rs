use crate::{handlers::log, models::types::OperType};

mod alias;
mod auto_check_update;
pub mod config_data;
mod list;

pub fn handler(statement: &Option<Vec<String>>) {
    // 下面开始判定具体改了什么设置
    if statement.is_none() {
        handle_show(&None);
        return;
    }

    let args = statement.as_ref().unwrap();
    // 只打印，不做修改
    if args.len() == 1 {
        let arg = args[0].to_string();
        handle_show(&Some(arg));
        return;
    }

    if args.len() != 2 {
        println!(
            "Invalid config statement. There must be 2 args. Got '{}'.",
            args.join(" ")
        );
        return;
    }

    let item = &args[0];
    let arg = &args[1];

    match item.as_str() {
        "alias.add" => handle_add_alias(arg),
        "alias.remove" => handle_remove_alias(arg),
        "auto-check-update" => handle_auto_check_update(arg),
        _ => {
            println!("Unknown config item: '{}'", item);
            return;
        }
    }
}

fn handle_show(config_item: &Option<String>) {
    if let Err(e) = list::show(config_item) {
        println!("{}", e.to_string());
    }
}

fn handle_add_alias(arg: &str) {
    let oper = OperType::Config {
        option: "alias.add".to_string(),
    };
    match alias::set_alias(&arg) {
        Ok(_) => {
            println!("Alias '{}' is set successfully.", arg);
            log::succ(&oper, arg, None, None);
        }
        Err(e) => log::err(&oper, arg, None, e),
    }
}

fn handle_remove_alias(arg: &str) {
    let oper = OperType::Config {
        option: "alias.remove".to_string(),
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
        option: "auto-check-update".to_string(),
    };

    match auto_check_update::toggle(&arg) {
        Ok(_) => {
            println!("Auto check update is set to '{}'.", arg);
            log::succ(&oper, arg, None, None);
        }
        Err(e) => log::err(&oper, arg, None, e),
    }
}
