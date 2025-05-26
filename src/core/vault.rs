use crate::{info, must_ok, must_some, wrap_result};

use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::config;
use crate::misc::{CustomColors, jsonl, paths};
use crate::models::error::ArchiverError;
use crate::models::types::{DEFAULT_VAULT_NAME, Vault};

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
        format!("vault_id: {} not found", id.colored_vault())
    );
    vault.name.clone()
}

/// 修改当前使用的 vault
pub fn use_by_name(name: &str) -> Result<u32, ArchiverError> {
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
pub fn create(
    name: &str,
    use_at_once: bool,
    remark: &Option<String>,
) -> Result<Vault, ArchiverError> {
    if let Some(vault) = find_by_name(name) {
        if vault.name == DEFAULT_VAULT_NAME {
            // 如果是默认库，则不允许创建同名库
            return info!(
                "'{}' means default vault, please choose another name",
                DEFAULT_VAULT_NAME
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
pub fn remove(name: &str) -> Result<u32, ArchiverError> {
    let vault = find_by_name(name);
    if vault.is_none() {
        return info!("Vault '{}' not found", name);
    }
    let vault = vault.unwrap();

    Ok(vault.id)
}
