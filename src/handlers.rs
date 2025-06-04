use owo_colors::OwoColorize;
use std::{cmp::Ordering, collections::HashSet};

use crate::cli::{ConfigAction, VaultAction};
use crate::core::config::CONFIG;
use crate::core::{archive, config, log, update, vault};
use crate::misc::{clap_mark, mark};
use crate::models::types::{DEFAULT_VLT_ID, ListEntry};
use crate::traits::{CustomColors, ResultExt};

pub fn vault(action: &VaultAction) {
    match action {
        VaultAction::Create {
            name,
            remark,
            activate,
        } => vault::create(name, *activate, remark).ok_then_or_log(|v| {
            let msg = format!(
                "Vault '{}' is successfully created, vault id: {}",
                name,
                v.id.styled_vault()
            );
            log::succ(None, v.id, &msg);
        }),
        VaultAction::List => vault::display(),
        VaultAction::Use { name } => vault::use_by_name(name).ok_then_or_log(|vault_id| {
            let msg = format!("Vault '{}' is successfully set as current vault", name);
            log::succ(None, vault_id, &msg);
        }),
        VaultAction::Remove { name } => {
            vault::use_by_name(name).ok_then_or_log(|vault_id| {
                let msg = format!("Vault '{}' is successfully removed", name);
                log::succ(None, vault_id, &msg);
            });
        }
    }
}

pub fn put(items: &Vec<String>, message: &Option<String>, vault: &Option<String>) {
    let vault_id = match vault {
        Some(name) => match vault::find_by_name(&name) {
            Some(v) => v.id,
            None => {
                log::fail(&format!("Vault '{}' not found", name));
                return;
            }
        },
        None => DEFAULT_VLT_ID, // 默认使用0号vault
    };

    if let Err(e) = archive::put_check(items) {
        e.display();
        return;
    }

    let mut count: usize = 0;
    items.iter().for_each(|item| {
        println!("Putting '{}' into archive", item);
        // 循环中使用message必须clone，否则move一次就没了
        archive::put(&item, message.clone(), vault_id).ok_then_or_log(|entry| {
            let msg = format!(
                "'{}' is successfully archived (id: {}, vault: {})",
                item,
                entry.id,
                vault::get_name(entry.vault_id),
            );
            log::succ(entry.id, entry.vault_id, &msg);
            count += 1;
        });
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
        println!("Restoring id: {}", id.styled_id());
        archive::restore(id).ok_then_or_log(|entry| {
            let msg = format!(
                "({}{}{}) is successfully restored to '{}'",
                entry.id.styled_id(),
                CONFIG.vault_item_sep.styled_vault_item_seperator(),
                vault::get_name(entry.vault_id).styled_vault(),
                entry.get_item_path_string()
            );
            log::succ(entry.id, entry.vault_id, &msg);
        });
    }
}

pub fn mv(ids: &[u32], to: &str) {
    // 去重以防止重复操作同一目标
    let vault_id = match vault::find_by_name(to) {
        Some(v) => v.id,
        None => {
            log::fail("Vault not found");
            return;
        }
    };

    let satisfies = |entry: &ListEntry| ids.contains(&entry.id) && entry.vault_id != vault_id;
    let count = match archive::batch_mv(satisfies, vault_id) {
        Ok(count) => {
            let msg = format!(
                "{} objects are successfully moved to vault '{}', id: {}",
                count,
                to,
                vault_id.styled_vault()
            );
            log::succ(None, vault_id, &msg);
            count
        }
        Err(e) => {
            log::error(e);
            println!("{} Please use `arv log` for details.", mark::info());
            return;
        }
    };

    // 如果没有任何对象被移动，输出错误信息
    if count == 0 {
        log::fail("No satisfied archived object found");
        return;
    }
}

pub fn list(all: bool, restored: bool) {
    archive::list::display(all, restored).allow_and_display();
}

pub fn log(range: &Option<String>, id: &Option<u32>) {
    if let Some(id) = id {
        if range.is_some() {
            println!(
                "{} No need to enter [range] when using `--id` option.",
                clap_mark::info()
            );
        }
        // 如果指定了id，则显示单条日志
        log::display_by_id(*id).allow_and_display();
    } else {
        log::display(range).allow_and_display();
    }
}

pub fn config(action: &ConfigAction) {
    match action {
        ConfigAction::List => config::display(),
        ConfigAction::Alias { entry, remove } => {
            if *remove {
                config::alias::remove(&entry).ok_then_or_log(|_| {
                    let msg = format!("Alias '{}' is removed successfully", entry);
                    log::succ(None, None, &msg);
                })
            } else {
                config::alias::add(entry).ok_then_or_log(|_| {
                    let msg = format!("Alias '{}' is added successfully", entry);
                    log::succ(None, None, &msg);
                })
            }
        }
        ConfigAction::UpdateCheck { status } => {
            config::update_check::set(&status).ok_then_or_log(|_| {
                let msg = if status == "on" {
                    format!("Update check is turned {}", status.green().bold())
                } else {
                    format!("Update check is turned {}", status.red().bold())
                };
                log::succ(None, None, &msg);
            })
        }
        ConfigAction::VaultItemSep { sep } => {
            config::vault_item_sep::set(sep).ok_then_or_log(|_| {
                let msg = format!("Vault-item separator is set to '{}'", sep);
                log::succ(None, None, &msg);
            })
        }
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
