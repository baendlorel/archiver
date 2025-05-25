use crate::{info, must_ok, must_some, wrap_result};

use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::config;
use crate::{
    misc::{jsonl, paths},
    models::{error::ArchiverError, types::Vault},
};

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

/// 根据vault_id获取vault名字，用于log、list等展示
pub fn get_name(id: u32) -> String {
    let vault = must_some!(
        VAULT_MAP.get(&id),
        format!("Vault with id:{} not found", id)
    );
    vault.name.clone()
}

/// 修改当前使用的 vault
pub fn use_by_name(name: &str) -> Result<u32, ArchiverError> {
    let vault = VAULT_MAP.iter().find(|(_, vault)| vault.name == name);
    if vault.is_none() {
        return info!("Vault '{}' not found", name);
    }

    // 更新current_vault_id
    let (id, _) = vault.unwrap();
    let mut config = wrap_result!(config::load())?;
    config.current_vault_id = *id;
    wrap_result!(config::save(&config))?;

    Ok(*id)
}

/// 创建一个新的 vault，不能重名
pub fn create(name: &str, remark: &Option<String>) -> Result<Vault, ArchiverError> {
    let vault = VAULT_MAP.iter().find(|(_, vault)| vault.name == name);
    if vault.is_some() {
        return info!("Vault with the same name '{}' already exists", name);
    }

    let vault = Vault::new(name, remark.clone());
    wrap_result!(jsonl::append(&vault, &paths::VAULTS_FILE_PATH))?;

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

pub fn remove(name: &str) -> Result<u32, ArchiverError> {
    let vault = VAULT_MAP.iter().find(|(_, vault)| vault.name == name);
    if vault.is_none() {
        return info!("Vault '{}' not found", name);
    }
    let (id, vault) = vault.unwrap();

    // todo 下面将vault标记为删除

    Ok(*id)
}
