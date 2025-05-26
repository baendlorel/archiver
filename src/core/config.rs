use crate::log_if_err;

use super::log;

mod alias;
pub mod auto_check_update;
mod command;
mod display;
mod sl;

pub use sl::{load, save};

pub fn display() {
    log_if_err!(display::display(&None));
}

pub fn add_alias(alias: &str) {
    match alias::set_alias(&alias) {
        Ok(_) => {
            let msg = format!("Alias '{}' is set successfully", alias);
            log::succ(None, None, &msg);
        }
        Err(e) => log::fail(e),
    }
}

pub fn remove_alias(alias: &str) {
    match alias::remove_alias(&alias) {
        Ok(_) => {
            let msg = format!("Alias '{}' is removed successfully", alias);
            log::succ(None, None, &msg);
        }
        Err(e) => log::fail(e),
    }
}

pub fn auto_check_update(status: &str) {
    match auto_check_update::toggle(&status) {
        Ok(_) => {
            let msg = format!("Auto check update is set to '{}'", status);
            log::succ(None, None, &msg);
        }
        Err(e) => log::fail(e),
    }
}
