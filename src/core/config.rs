use crate::log_if_err;

use super::log;
use crate::models::types::OperType;

mod alias;
pub mod auto_check_update;
mod command;
mod display;
mod sl;

pub use command::{ConfigCommand, parse_command};
pub use sl::{load, save};

pub fn display(config_item: &Option<String>) {
    log_if_err!(display::display(config_item));
}

pub fn add_alias(arg: &str) {
    let oper = OperType::Config("alias.add".to_string());
    match alias::set_alias(&arg) {
        Ok(_) => {
            let msg = format!("Alias '{}' is set successfully.", arg);
            log::succ(&oper, arg, None, &msg);
        }
        Err(e) => log::err(&oper, arg, None, e),
    }
}

pub fn remove_alias(arg: &str) {
    let oper = OperType::Config("alias.remove".to_string());

    match alias::remove_alias(&arg) {
        Ok(_) => {
            let msg = format!("Alias '{}' is removed successfully.", arg);
            log::succ(&oper, arg, None, &msg);
        }
        Err(e) => log::err(&oper, arg, None, e),
    }
}

pub fn auto_check_update(arg: &str) {
    let oper = OperType::Config("auto-check-update".to_string());

    match auto_check_update::toggle(&arg) {
        Ok(_) => {
            let msg = format!("Auto check update is set to '{}'.", arg);
            log::succ(&oper, arg, None, &msg);
        }
        Err(e) => log::err(&oper, arg, None, e),
    }
}
