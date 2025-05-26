use crate::{err_info, err_warn, log_if_err, must_ok};

use owo_colors::OwoColorize;
use std::{cmp::Ordering, collections::HashSet};

use crate::{
    cli::VaultAction,
    core::{archive, config, log, update, vault},
    misc::mark,
};

pub fn vault(action: &VaultAction) {
    match action {
        VaultAction::Create {
            name,
            remark,
            u: use_at_once,
        } => match vault::create(name, *use_at_once, remark) {
            Ok(vault) => {
                let msg = format!(
                    "Vault '{}' is successfully created, vault id:{}",
                    name, vault.id
                );
                let arg = log::format_arg::vault::create(name, *use_at_once, remark);
                log::succ(None, Some(vault.id), &msg);
            }
            Err(e) => log::fail(e),
        },
        VaultAction::List => vault::display(),
        VaultAction::Use { name } => match vault::use_by_name(name) {
            Ok(vault_id) => {
                let msg = format!("Vault '{}' is successfully set as current vault", name);
                log::succ(None, Some(vault_id), &msg);
            }
            Err(e) => {
                log::fail(e);
            }
        },
        VaultAction::Remove { name } => match vault::remove(name) {
            Ok(vault_id) => {
                let msg = format!("Vault '{}' is successfully removed", name);
                log::succ(None, Some(vault_id), &msg);
            }
            Err(e) => {
                log::fail(e);
            }
        },
    }
}

pub fn put(targets: &[String], message: &Option<String>) {
    // 去重以防止重复操作同一目标
    let set: HashSet<String> = targets.iter().cloned().collect();
    for target in set {
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
                log::succ(Some(entry.id), Some(entry.vault_id), &msg);
            }
            Err(e) => log::fail(e),
        };
    }
    println!("Use `arv list` to check the archived list");
}

pub fn restore(ids: &[u32]) {
    // 去重以防止重复操作同一目标
    let set: HashSet<u32> = ids.iter().cloned().collect();
    for id in set {
        println!("Restoring id:{}", id);
        match archive::restore(id) {
            Ok(entry) => {
                let msg = format!(
                    "id:{} (vlt:{}) is successfully restored to '{}'",
                    entry.id,
                    vault::get_name(entry.vault_id),
                    entry.get_target_path_string()
                );
                log::succ(Some(entry.id), Some(entry.vault_id), &msg);
            }
            Err(e) => log::fail(e),
        }
    }
}

pub fn move_to(ids: &[u32], to: &str) {
    // 去重以防止重复操作同一目标
    let vault_id = match vault::find_by_name(to) {
        Some(v) => v.id,
        None => {
            log::fail(err_warn!("Vault not found"));
            return;
        }
    };
    let mut full_list = match archive::sl::load() {
        Ok(list) => list,
        Err(e) => {
            log::fail(e);
            return;
        }
    };

    let set: HashSet<u32> = ids.iter().cloned().collect();
    let mut count = 0;

    for entry in full_list.iter_mut() {
        if !set.contains(&entry.id) || entry.vault_id != vault_id {
            continue; // 跳过不在ids中的id
        }
        match archive::do_the_move(&entry, vault_id) {
            Ok(_) => {
                count += 1;
                entry.vault_id = vault_id; // 更新目标vault_id
                let msg = format!("id:{} is moved to '{}'", entry.id, to);
                log::succ(None, None, &msg);
            }
            Err(e) => log::fail(e),
        }
    }

    // 如果没有任何对象被移动，输出错误信息
    if count == 0 {
        let e = err_info!("No satisfied archived object found");
        log::fail(e);
        return;
    }

    must_ok!(archive::sl::save(&full_list), "");

    if count > 1 {
        // 当移动了超过1条记录，则写一条总结日志
        let msg = format!("{}/{} archived objects moved to '{}'", count, set.len(), to);
        log::succ(None, None, &msg);
    }
}

pub fn list(all: bool, restored: bool) {
    log_if_err!(archive::list::display(all, restored));
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
