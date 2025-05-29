use crate::{info, must_ok, must_some, wrap_result};

use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use std::collections::HashMap;

use super::config;
use crate::core::archive;
use crate::misc::{console, jsonl, paths, rand};
use crate::models::error::ArchiverResult;
use crate::models::types::{DEFAULT_VLT_ID, DEFAULT_VLT_NAME};
use crate::models::types::{ListEntry, ListStatus, Vault, VaultStatus};
use crate::traits::CustomColors;

static VAULT_MAP: Lazy<HashMap<u32, Vault>> = Lazy::new(|| {
    let vaults = must_ok!(
        jsonl::load::<Vault>(&paths::VAULTS_FILE_PATH),
        "Failed to load vaults data"
    );

    let mut vault_map: HashMap<u32, Vault> = HashMap::new();
    vault_map.insert(0, Vault::default()); // 默认vault
    for v in vaults {
        vault_map.insert(v.id, v);
    }

    vault_map
});

/// 根据name搜索的，一般只要搜一条就行，但根据vid搜却可能要几百次（例如LogEntry），所以id做键
pub fn find_by_name(name: &str) -> Option<Vault> {
    if let Some((_, vault)) = VAULT_MAP.iter().find(|(_, vault)| vault.name == name) {
        Some(vault.clone())
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
    let vault = find_by_name(name);
    if vault.is_none() {
        return info!("Vault '{}' not found", name);
    }

    // 更新current_vault_id
    let id = vault.unwrap().id;
    let mut config = wrap_result!(config::load())?;
    config.current_vault_id = id;
    wrap_result!(config::save(&config))?;

    Ok(id)
}

/// 创建一个新的 vault，不能重名
pub fn create(name: &str, use_at_once: bool, remark: &Option<String>) -> ArchiverResult<Vault> {
    if let Some(vault) = find_by_name(name) {
        if vault.name == DEFAULT_VLT_NAME {
            // 如果是默认库，则不允许创建同名库
            return info!(
                "'{}' means default vault, please choose another name",
                DEFAULT_VLT_NAME
            );
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

    if use_at_once {
        // 如果需要立即使用这个vault
        wrap_result!(use_by_name(name))?;
    }

    // 此处不需要VAULTS.push(vault)，因为创建结束后就退出了
    Ok(vault)
}

pub fn display() {
    VAULT_MAP.iter().for_each(|(id, vault)| {
        println!(
            "{} {} {} {}",
            id, vault.name, vault.remark, vault.created_at
        );
    });
}

// todo 删除一个vault
/// 根据名字删除一个vault
/// - 其中的归档对象会被转移到default库
pub fn remove(name: &str) -> ArchiverResult<u32> {
    let mut vaults = VAULT_MAP
        .iter()
        .map(|(_, v)| v.clone())
        .collect::<Vec<Vault>>();
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
        "All archived objects in '{}' {}{}{} (which is the default vault).",
        name.styled_vault(),
        "shall be moved to '".underline().bold(),
        DEFAULT_VLT_NAME.styled_vault().underline().bold(),
        "'".underline().bold()
    );
    if !console::confirm("Are you sure?") {
        return info!("Removal cancelled");
    }

    // 删除前确认
    let verify_code = rand::string(6);
    println!(
        "To confirm removing vault '{}', please type: {}",
        name.styled_vault(),
        verify_code
    );
    if !console::confirm_str("> ", &verify_code) {
        return info!("Confirmation failed, exit");
    }

    // * 删除
    // 移动归档对象到默认库
    // 已回收的就不管了
    let satisfies = |entry: &ListEntry| {
        matches!(entry.status, ListStatus::Archived) && entry.vault_id == vaults[index].id
    };

    wrap_result!(archive::batch_mv(satisfies, DEFAULT_VLT_ID))?;

    // 修改vaults.jsonl
    vaults[index].status = VaultStatus::Removed;
    wrap_result!(jsonl::save(&vaults, paths::VAULTS_FILE_PATH.as_path()))?;

    Ok(vaults[index].id)
}
