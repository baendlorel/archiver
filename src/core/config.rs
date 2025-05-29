use crate::allow;

use super::log;

mod alias;
mod display;
mod sl;

pub mod auto_check_update;
pub use sl::{load, save};

pub fn display() {
    allow!(display::display(&None));
}

pub fn add_alias(alias: &str) {
    match alias::set_alias(&alias) {
        Ok(_) => {
            let msg = format!("Alias '{}' is set successfully", alias);
            log::succ(None, None, &msg);
        }
        Err(e) => log::error(e),
    }
}

pub fn remove_alias(alias: &str) {
    match alias::remove_alias(&alias) {
        Ok(_) => {
            let msg = format!("Alias '{}' is removed successfully", alias);
            log::succ(None, None, &msg);
        }
        Err(e) => log::error(e),
    }
}

pub fn auto_check_update(status: &str) {
    match auto_check_update::toggle(&status) {
        Ok(_) => {
            let msg = format!("Auto check update is set to '{}'", status);
            log::succ(None, None, &msg);
        }
        Err(e) => log::error(e),
    }
}
