use crate::{allow, err_info, err_warn};

use owo_colors::OwoColorize;
use std::{cmp::Ordering, collections::HashSet};

use crate::cli::{AliasAction, AutoCheckUpdateAction, ConfigAction, VaultAction};
use crate::core::{archive, config, log, update, vault};
use crate::misc::{dedup_and_log, mark};
use crate::models::types::{DEFAULT_VLT_ID, ListEntry};
use crate::traits::CustomColors;

pub fn vault(action: &VaultAction) {
    match action {
        VaultAction::Create {
            name,
            remark,
            u: use_at_once,
        } => match vault::create(name, *use_at_once, remark) {
            Ok(vault) => {
                let msg = format!(
                    "Vault '{}' is successfully created, vault id: {}",
                    name,
                    vault.id.styled_vault()
                );
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
            Err(e) => log::fail(e),
        },
        VaultAction::Remove { name } => match vault::remove(name) {
            Ok(vault_id) => {
                let msg = format!("Vault '{}' is successfully removed", name);
                log::succ(None, Some(vault_id), &msg);
            }
            Err(e) => log::fail(e),
        },
    }
}

pub fn put(items: &Vec<String>, message: &Option<String>, vault: &Option<String>) {
    let vault_id = match vault {
        Some(name) => match vault::find_by_name(&name) {
            Some(v) => v.id,
            None => {
                log::fail(err_warn!("Vault '{}' not found", name));
                return;
            }
        },
        None => DEFAULT_VLT_ID, // 默认使用0号vault
    };

    // 去重以防止重复操作同一目标
    let items = dedup_and_log(items);

    let mut count: usize = 0;
    items.iter().for_each(|item| {
        println!("Putting '{}' into archive", item);
        // 循环中使用message必须clone，否则move一次就没了
        match archive::put(&item, message.clone(), vault_id) {
            Ok(entry) => {
                let msg = format!(
                    "'{}' is successfully archived (id: {}, vault: {})",
                    item,
                    entry.id,
                    vault::get_name(entry.vault_id),
                );
                log::succ(Some(entry.id), Some(entry.vault_id), &msg);
                count += 1;
            }
            Err(e) => log::fail(e),
        };
    });

    if items.len() > 1 {
        println!("{}/{} items are successfully archived", count, items.len());
    }
    println!("Use `arv list` to check the archived list");
}

pub fn restore(ids: &[u32]) {
    // 去重以防止重复操作同一目标
    let set: HashSet<u32> = ids.iter().cloned().collect();
    for id in set {
        println!("Restoring id: {}", id.styled_archive_id());
        match archive::restore(id) {
            Ok(entry) => {
                let msg = format!(
                    "(id: {}, vault: {}) is successfully restored to '{}'",
                    entry.id.styled_archive_id(),
                    vault::get_name(entry.vault_id).styled_vault(),
                    entry.get_item_path_string()
                );
                log::succ(Some(entry.id), Some(entry.vault_id), &msg);
            }
            Err(e) => log::fail(e),
        }
    }
}

pub fn mv(ids: &[u32], to: &str) {
    // 去重以防止重复操作同一目标
    let vault_id = match vault::find_by_name(to) {
        Some(v) => v.id,
        None => {
            log::fail(err_info!("Vault not found"));
            return;
        }
    };

    let satisfies = |entry: &ListEntry| ids.contains(&entry.id) && entry.vault_id != vault_id;
    let count = match archive::batch_mv(satisfies, vault_id) {
        Ok(count) => {
            let msg = format!("{} objects are successfully moved to vault '{}'", count, to);
            log::succ(None, Some(vault_id), &msg);
            count
        }
        Err(e) => {
            log::fail(e);
            println!("{} Please use `arv log` for details.", mark::info());
            return;
        }
    };

    // 如果没有任何对象被移动，输出错误信息
    if count == 0 {
        let e = err_info!("No satisfied archived object found");
        log::fail(e);
        return;
    }
}

pub fn list(all: bool, restored: bool) {
    allow!(archive::list::display(all, restored));
}

pub fn log(range: &Option<String>) {
    allow!(log::display(range));
}

pub fn config(action: &ConfigAction) {
    match action {
        ConfigAction::List => config::display(),
        ConfigAction::Alias(action) => match action {
            AliasAction::Add { alias } => config::add_alias(&alias),
            AliasAction::Remove { alias } => config::remove_alias(&alias),
        },
        ConfigAction::AutoCheckUpdate(action) => match action {
            AutoCheckUpdateAction::Set { status } => config::auto_check_update(status),
        },
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
