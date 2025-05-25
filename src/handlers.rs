use crate::log_if_err;

use owo_colors::OwoColorize;
use std::cmp::Ordering;

use crate::{
    cli::VaultAction,
    core::{archive, config, list, log, update, vault},
    misc::mark,
    models::types::OperType,
};

pub fn vault(action: &VaultAction) {
    match action {
        VaultAction::Create {
            name,
            remark,
            u: use_at_once,
        } => {
            let oper = OperType::Vault("create".to_string());
            match vault::create(name, *use_at_once, remark) {
                Ok(vault) => {
                    let msg = format!(
                        "Vault '{}' is successfully created, vault id:{}",
                        name, vault.id
                    );
                    let arg = log::format_arg::vault::create(name, *use_at_once, remark);
                    log::succ(&oper, &arg, None, Some(vault.id), &msg);
                }
                Err(e) => log::err(&oper, name, e),
            }
        }
        VaultAction::List => vault::display(),
        VaultAction::Use { name } => {
            let oper = OperType::Vault("use".to_string());
            match vault::use_by_name(name) {
                Ok(vault_id) => {
                    let msg = format!("Vault '{}' is successfully set as current vault", name);
                    log::succ(&oper, name, None, Some(vault_id), &msg);
                }
                Err(e) => {
                    log::err(&oper, name, e);
                }
            }
        }
        VaultAction::Remove { name } => {
            let oper = OperType::Vault(format!("remove {}", name));
            match vault::remove(name) {
                Ok(vault_id) => {
                    let msg = format!("Vault '{}' is successfully removed", name);
                    log::succ(&oper, name, None, Some(vault_id), &msg);
                }
                Err(e) => {
                    log::err(&oper, name, e);
                }
            }
        }
    }
}

pub fn put(targets: &[String], message: &Option<String>) {
    let oper = OperType::Put;
    for target in targets {
        println!("Putting '{}' into archive", target);
        match archive::put(&target, message) {
            Ok(entry) => {
                let msg = format!(
                    "'{}' is successfully archived, id:{} (vlt:{}), message: {}",
                    target,
                    entry.id,
                    vault::get_name(entry.vault_id),
                    entry.message,
                );
                log::succ(&oper, target, Some(entry.id), Some(entry.vault_id), &msg);
            }
            Err(e) => log::err(&oper, target, e),
        };
    }
    println!("Use `arv list` to check the archived list");
}

pub fn restore(ids: &[u32]) {
    let oper = OperType::Restore;
    for id in ids {
        println!("Restoring id:{}", id);
        match archive::restore(*id) {
            Ok(entry) => {
                let msg = format!(
                    "id:{} (vlt:{}) is successfully restored to '{}'",
                    entry.id,
                    vault::get_name(entry.vault_id),
                    entry.get_target_path_string()
                );
                log::succ(
                    &oper,
                    &id.to_string(),
                    Some(entry.id),
                    Some(entry.vault_id),
                    &msg,
                );
            }
            Err(e) => log::err(&oper, &id.to_string(), e),
        }
    }
}

pub fn move_to(ids: &[u32], to: &str) {
    let oper = OperType::Move;
    match archive::move_to(ids, to) {
        Ok((count, total)) => {
            let msg = format!(
                "{}/{} entries are successfully moved to vault '{}'",
                count, total, to
            );
            log::succ(&oper, &to.to_string(), None, None, &msg);
        }
        Err(e) => log::err(&oper, &to.to_string(), e),
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
