use crate::{info, must_ok, must_some, oper, opt_map, wrap_result};

use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use std::collections::HashMap;

use super::config;
use crate::cli::short::main;
use crate::core::{archive, log};
use crate::misc::console::{confirm, table::Table};
use crate::misc::{clap_mark, console, jsonl, paths, rand};
use crate::models::error::ArchiverResult;
use crate::models::types::{ListEntry, ListStatus, LogLevel, Vault, VaultStatus, vault_defaults};
use crate::traits::CustomColors;

static VAULT_LIST: Lazy<Vec<Vault>> = Lazy::new(|| {
    let mut vaults = must_ok!(
        jsonl::load::<Vault>(&paths::VAULTS_FILE_PATH),
        "Failed to load vaults data"
    );
    vaults.insert(0, Vault::default()); // 默认库
    vaults
});

static VAULT_MAP: Lazy<HashMap<u32, Vault>> = Lazy::new(|| {
    let m: HashMap<u32, Vault> = HashMap::from_iter(VAULT_LIST.iter().map(|v| (v.id, v.clone())));
    m
});

static VAULT_NAME_MAP: Lazy<HashMap<String, Vault>> = Lazy::new(|| {
    let m: HashMap<String, Vault> =
        HashMap::from_iter(VAULT_LIST.iter().map(|v| (v.name.clone(), v.clone())));
    m
});

pub fn find(condition: impl Fn(&Vault) -> bool) -> Vec<Vault> {
    VAULT_LIST
        .iter()
        .filter(|&v| condition(v))
        .cloned()
        .collect::<Vec<Vault>>()
}

/// 根据name搜索的，一般只要搜一条就行，但根据vid搜却可能要几百次（例如LogEntry），所以id做键
pub fn get_id(name: &str) -> Option<u32> {
    if let Some(vault) = VAULT_NAME_MAP.get(name) {
        Some(vault.id)
    } else {
        None
    }
}

/// 根据vault_id获取vault名字，用于log、list等展示
pub fn get_name(id: u32) -> String {
    let vault = must_some!(
        VAULT_MAP.get(&id),
        format!("vault_id: {} not found", id.styled_vault())
    );
    vault.name.clone()
}

/// 修改当前使用的 vault
pub fn use_by_name(name: &str) -> ArchiverResult<u32> {
    let id: Option<u32> = get_id(name);
    if id.is_none() {
        return info!("Vault '{}' not found", name);
    }

    // 更新current_vault_id
    let id = id.unwrap();
    let mut config = config::CONFIG.clone();
    config.current_vault_id = id;
    wrap_result!(config::save(&config))?;

    Ok(id)
}

/// 创建一个新的 vault，不能重名
pub fn create(name: &str, activate: bool, remark: &Option<String>) -> ArchiverResult<Vault> {
    if let Some(vault) = VAULT_NAME_MAP.get(name) {
        // 如果是默认库，则不允许创建和默认库同名的库
        if vault.id == vault_defaults::ID {
            return info!(
                "'{}' means default vault, please choose another name",
                vault_defaults::NAME
            );
        }

        if matches!(vault.status, VaultStatus::Removed) {
            // 如果是已删除的库，则可以恢复
            if confirm(&format!(
                "'{}' is a removed vault, do you want to recover it?",
                name
            )) {
                wrap_result!(recover(name))?;
                log::trans(
                    oper![main::VAULT, "restore", vec![name], None, "trans"],
                    LogLevel::Success,
                    None,
                    vec![vault.id],
                );
            } else {
                println!("{} recovery cancelled", clap_mark::info());
            }
            // 直接退出，同时也是为了避免日志记录奇怪
            std::process::exit(0);
        }

        return info!(
            "Vault named '{}' already exists, please choose another name",
            name
        );
    }

    let vault = Vault::new(name, remark.clone());
    wrap_result!(jsonl::append(&vault, &paths::VAULTS_FILE_PATH))?;

    // 此函数可以用于创建
    let _ = paths::get_vault_path(vault.id);

    if activate {
        // 如果需要立即使用这个vault
        wrap_result!(use_by_name(name))?;
    }

    // 此处不需要VAULTS.push(vault)，因为创建结束后就退出了
    Ok(vault)
}

/// 根据名字删除一个vault
/// - 其中的归档对象会被转移到default库
pub fn remove(name: &str) -> ArchiverResult<Vault> {
    let mut vaults = VAULT_LIST.clone();
    let index = vaults
        .iter()
        .position(|v| matches!(v.status, VaultStatus::Valid) && v.name == name);
    if index.is_none() {
        return info!("Vault '{}' not found", name);
    }

    // * 能找到，那么下面开始删除
    let index = index.unwrap();

    // 告知删除会导致归档对象移动到默认库
    println!(
        "All archived items in '{}' {}{}{} (which is the default vault).",
        name.styled_vault(),
        "shall be moved to '".underline().bold(),
        vault_defaults::NAME.styled_vault().underline().bold(),
        "'".underline().bold()
    );
    if !console::confirm("Are you sure?") {
        return info!("Removal cancelled");
    }

    // 删除前确认
    let verify_code = rand::string(4);
    print!(
        "To confirm removing, please type: {} ",
        verify_code.yellow().bold()
    );

    if !console::confirm_str("> ", &verify_code) {
        return info!("Confirmation failed, exit");
    }

    // * 删除
    // 移动归档对象到默认库
    // 已回收的就不管了
    let list = wrap_result!(jsonl::load::<ListEntry>(paths::LIST_FILE_PATH.as_path()))?;
    let ids = list
        .iter()
        .filter(|entry| {
            matches!(entry.status, ListStatus::Archived) && entry.vault_id == vaults[index].id
        })
        .map(|entry| entry.id)
        .collect::<Vec<u32>>();

    if ids.len() > 0 {
        wrap_result!(archive::mov_check(&ids, vault_defaults::ID))?;

        // 下面开始mov，过程类似于handlers中的mov，但是是精简版
        let mut count = 0;
        let to = vault_defaults::NAME;
        let styled_to = to.styled_vault();
        let succ = clap_mark::succ();
        for id in &ids {
            println!("Moving id: {} into {}", id.styled_id(), styled_to);

            let oper = oper!(main::MOVE, None, [id], opt_map![to], "sys");
            match archive::mov(*id, vault_defaults::ID) {
                Ok(_) => {
                    println!("{} Id: {} is now in '{}'", succ, id.styled_id(), styled_to);
                    log::sys(oper, LogLevel::Success, vec![*id], vec![vault_defaults::ID]);
                    count += 1;
                }
                Err(e) => {
                    e.display();
                    log::sys(oper, e.level, vec![*id], vec![vault_defaults::ID]);
                }
            }
        }

        println!(
            "{} {}/{} items are successfully moved to vault '{}'",
            clap_mark::succ(),
            count,
            ids.len(),
            styled_to
        );
    }

    // 修改vaults.jsonl
    vaults[index].status = VaultStatus::Removed;

    // 这里要防止默认库也被写入文件
    let vaults_exclude_default = vaults
        .iter()
        .filter(|v| v.id != vault_defaults::ID)
        .cloned()
        .collect::<Vec<Vault>>();

    wrap_result!(jsonl::save(
        &vaults_exclude_default,
        paths::VAULTS_FILE_PATH.as_path()
    ))?;

    Ok(vaults[index].clone())
}

pub fn recover(name: &str) -> ArchiverResult<Vault> {
    let mut vaults = VAULT_LIST.clone();
    let index = vaults
        .iter()
        .position(|v| matches!(v.status, VaultStatus::Removed) && v.name == name);
    if index.is_none() {
        return info!("No removed vault '{}' was found", name);
    }

    // * 能找到，那么下面开始恢复
    let index = index.unwrap();
    vaults[index].status = VaultStatus::Valid;

    // 修改vaults.jsonl
    wrap_result!(jsonl::save(&vaults, paths::VAULTS_FILE_PATH.as_path()))?;

    Ok(vaults[index].clone())
}

pub fn display(all: bool) {
    let vaults = VAULT_LIST
        .iter()
        .filter(|v| all || matches!(v.status, VaultStatus::Valid))
        .cloned()
        .collect::<Vec<Vault>>();

    Table::display(&vaults);
}
