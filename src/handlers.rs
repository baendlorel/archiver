use crate::log_if_err;

use owo_colors::OwoColorize;
use std::cmp::Ordering;

use crate::{
    core::{archive, config, list, log, update, vault},
    misc::mark,
    models::types::OperType,
};

pub fn vault(name: &Option<String>) {}

pub fn put(targets: &[String]) {
    let oper = OperType::Put;
    for target in targets {
        println!("Putting '{}' into archive", target);
        match archive::put(&target) {
            Ok(id) => {
                let msg = format!("'{}' is successfully archived, id:{}", target, id);
                log::succ(&oper, target, Some(id), &msg);
            }
            Err(e) => log::err(&oper, target, None, e),
        };
    }
    println!("Use `arv list` to check the archived list");
}

pub fn restore(ids: &[u32]) {
    let oper = OperType::Restore;
    for id in ids {
        println!("Restoring id:{}", id.magenta());
        match archive::restore(*id) {
            Ok(entry) => {
                let msg = format!(
                    "id:{} is successfully restored to '{}'",
                    id.magenta(),
                    entry.get_target_path()
                );
                log::succ(&oper, &id.to_string(), Some(*id), &msg);
            }
            Err(e) => log::err(&oper, &id.to_string(), Some(*id), e),
        }
    }
}

pub fn list(all: bool, restored: bool) {
    log_if_err!(list::display(all, restored));
}

pub fn log(range: &Option<String>) {
    log_if_err!(log::display(range));
}

pub fn config(statement: &Option<Vec<String>>) {
    let cmd = config::parse_command(statement);
    match cmd {
        config::ConfigCommand::Display { item } => config::display(&item),
        config::ConfigCommand::Alias { add, remove } => {
            if let Some(arg) = add {
                config::add_alias(&arg)
            }
            if let Some(arg) = remove {
                config::remove_alias(&arg)
            }
        }
        config::ConfigCommand::AutoCheckUpdate { set } => config::auto_check_update(&set),
    }
}

pub fn update() {
    // 获取当前版本
    let (current, latest) = match update::prepare_versions() {
        Ok(v) => v,
        Err(e) => {
            e.display();
            return;
        }
    };

    println!("Current version: {}", current.to_string().cyan());
    println!("Latest  release: {}", latest.to_string().green());

    match latest.cmp(&current) {
        Ordering::Greater => {
            println!("{} New version available! Now updating...", mark::warn());
            update::reinstall();
        }
        Ordering::Equal => println!("{} You are using the latest version.", mark::succ()),
        Ordering::Less => println!("{} How could you use a newer version?", mark::warn()),
    }
}
