/// ^ 批量处理日志规则，可能要支持日志一次加好多行这样
/// count=0
///   输出完全失败，直接返回
/// len=1, count=1
///   不总结，输出单条user的
/// len>1, count>0
///   输出单条sys，一条总结
/// ^ 现行处理顺序
/// 1. 设置is_sys为数组长度是否>1，count设为0
/// 2. ok_then_or_log中，成功时打印相同的succ!
///     - 先进行succ!打印，再区分is_sys记录日志，最后count自增
/// 3. 总结
///     - 如果count==0，输出失败信息
///     - 如果len>1，输出总结信息
use crate::{oper, opt_map};

use owo_colors::OwoColorize;
use std::cmp::Ordering;
use std::vec;

use crate::cli::short::main;
use crate::cli::{ConfigAction, VaultAction};
use crate::core::{archive, check, config, log, update, vault};
use crate::misc::clap_mark;
use crate::models::types::LogLevel;
use crate::traits::{CustomColors, ResultExt};

macro_rules! succ {
    ($s:expr)=>{
        println!("{} {}", clap_mark::succ(), $s)
    };
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        println!("{} {}", clap_mark::succ(), s)
    }};
}

pub fn vault(action: &VaultAction) {
    match action {
        VaultAction::Use { name } => vault::use_by_name(name).ok_then_or_log(|vault_id| {
            succ!("Vault '{}' is successfully set as current vault", name);
            log::succ(None, vec![vault_id]);
        }),
        VaultAction::Create {
            name,
            remark,
            activate,
        } => vault::create(name, *activate, remark).ok_then_or_log(|v| {
            succ!(
                "Vault '{}' is successfully created, vault id: {}",
                name,
                v.id.styled_vault()
            );
            log::succ(None, vec![v.id]);
        }),
        VaultAction::Remove { name } => {
            vault::remove(name).ok_then_or_log(|vault| {
                succ!("Vault '{}' is removed", name);
                log::succ(None, vec![vault.id]);
            });
        }
        VaultAction::Recover { name } => {
            vault::recover(name).ok_then_or_log(|vault| {
                succ!("Vault '{}' is recovered", name);
                log::succ(None, vec![vault.id]);
            });
        }
        VaultAction::List { all } => vault::display(*all),
    }
}

pub fn put(items: &Vec<String>, message: &Option<String>, vault: &Option<String>) {
    let vault_id = match vault {
        Some(name) => match vault::get_id(&name) {
            Some(id) => id,
            None => {
                log::fail(&format!("Vault '{}' not found", name));
                return;
            }
        },
        None => config::CONFIG.current_vault_id, // 默认使用config配置里的
    };

    if let Err(e) = archive::put_check(items, vault_id) {
        e.display();
        return;
    }

    // 校验结束，开始处理
    let is_sys = items.len() > 1;
    let mut succ_ids: Vec<u32> = vec![];
    for item in items {
        println!("Putting '{}' into archive", item);
        let oper = oper!(main::PUT, None, [item], opt_map![message, vault], "sys");
        // 循环中使用message必须clone，否则move一次就没了
        match archive::put(&item, message.clone(), vault_id) {
            Ok(entry) => {
                succ!(
                    "'{}' is now archived (id: {}, vault: {})",
                    item,
                    entry.id.styled_id(),
                    vault::get_name_styled(vault_id),
                );
                if is_sys {
                    log::sys(oper, LogLevel::Success, vec![entry.id], vec![vault_id]);
                } else {
                    log::succ(vec![entry.id], vec![vault_id]);
                }
                succ_ids.push(entry.id);
            }
            Err(e) => {
                if is_sys {
                    e.display();
                    log::sys(oper, e.level, None, vec![vault_id]);
                } else {
                    log::error(e);
                }
            }
        }
    }

    if succ_ids.len() == 0 {
        log::fail("No items were put. Please check your inputs.");
        return;
    }

    if items.len() > 1 {
        succ!(
            "{}/{} items are successfully archived",
            succ_ids.len(),
            items.len()
        );
        log::succ(succ_ids, vec![vault_id]);
    }
}

pub fn restore(ids: &[u32]) {
    if let Err(e) = archive::restore_check(ids) {
        return e.display();
    }

    // 校验结束，开始处理
    let is_sys = ids.len() > 1;
    let mut count = 0;
    for id in ids {
        println!("Restoring id: {}", id.styled_id());

        let oper = oper!(main::RESTORE, None, [id], None, "sys");
        match archive::restore(*id) {
            Ok(entry) => {
                succ!(
                    "{}{}{}({}) is restored to '{}'",
                    vault::get_name_styled(entry.vault_id),
                    config::CONFIG.vault_item_sep.styled_vault_item_sep(),
                    entry.id.styled_id(),
                    entry.item,
                    entry.get_item_path_string()
                );
                if is_sys {
                    log::sys(oper, LogLevel::Success, vec![*id], None);
                } else {
                    log::succ(vec![*id], None);
                }
                count += 1;
            }
            Err(e) => {
                if is_sys {
                    e.display();
                    log::sys(oper, e.level, None, None);
                } else {
                    log::error(e);
                }
            }
        }
    }

    if count == 0 {
        log::fail("No items were restored. Please check the ids.");
        return;
    }

    if ids.len() > 1 {
        succ!("{}/{} are restored", count, ids.len());
        log::succ(Vec::from(ids), None);
    }
}

pub fn mov(ids: &[u32], to: &str) {
    let vault_id = match vault::get_id(to) {
        Some(id) => id,
        None => {
            log::fail("Vault not found");
            return;
        }
    };

    if let Err(e) = archive::mov_check(ids, vault_id) {
        e.display();
        return;
    }

    // 校验结束，开始处理
    let is_sys = ids.len() > 1;
    let mut count = 0;
    for id in ids {
        println!("Moving id: {} into {}", id.styled_id(), to.styled_vault());
        let oper = oper!(main::MOVE, None, [id], opt_map![to], "sys");
        match archive::mov(*id, vault_id) {
            Ok(_) => {
                succ!("Id: {} is now in '{}'", id.styled_id(), to.styled_vault());
                if is_sys {
                    log::sys(oper, LogLevel::Success, vec![*id], vec![vault_id]);
                } else {
                    // 此分支只可能在总数为1，且成功1个的时候进入
                    log::succ(vec![*id], vec![vault_id]);
                }
                count += 1;
            }
            Err(e) => {
                if is_sys {
                    e.display();
                    log::sys(oper, e.level, vec![*id], vec![vault_id]);
                } else {
                    log::error(e);
                }
            }
        }
    }

    if count == 0 {
        log::fail("No items were moved. Please check the ids and vault name.");
        return;
    }

    if ids.len() > 1 {
        succ!(
            "{}/{} items are successfully moved to vault '{}'",
            count,
            ids.len(),
            to.styled_vault(),
        );
        log::succ(Vec::from(ids), vec![vault_id]);
    }
}

pub fn list(all: bool, restored: bool, vault: &Option<String>) {
    archive::list::display(all, restored, vault).allow_and_display();
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
        ConfigAction::List { comment } => config::display(*comment),
        ConfigAction::Alias { entry, remove } => {
            if *remove {
                config::alias::remove(&entry).ok_then_or_log(|_| {
                    succ!("Alias '{}' is removed successfully", entry);
                    log::succ(None, None);
                })
            } else {
                config::alias::add(entry).ok_then_or_log(|_| {
                    succ!("Alias '{}' is added successfully", entry);
                    log::succ(None, None);
                })
            }
        }
        ConfigAction::UpdateCheck { status } => {
            config::update_check::set(&status).ok_then_or_log(|_| {
                if status == "on" {
                    succ!("Update check is turned {}", status.green().bold())
                } else {
                    succ!("Update check is turned {}", status.red().bold())
                }
                log::succ(None, None);
            })
        }
        ConfigAction::VaultItemSep { sep } => {
            config::vault_item_sep::set(sep).ok_then_or_log(|_| {
                succ!("Vault-item separator is set to '{}'", sep);
                log::succ(None, None);
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
            println!(
                "{} New version available! Now updating...",
                clap_mark::warn()
            );
            update::reinstall();
        }
        Ordering::Equal => println!("{} You are using the latest version.", clap_mark::succ()),
        Ordering::Less => println!("{} How could you use a newer version?", clap_mark::warn()),
    }
}

pub fn check(verbose: &bool) {
    check::check(*verbose);
}
