use crate::{err_info, uoe_option, uoe_result, wrap_result};

use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::config;
use crate::{
    misc::{auto_incr, dt, jsonl, paths},
    models::{error::ArchiverError, types::Vault},
};

static VAULT_MAP: Lazy<HashMap<u32, Vault>> = Lazy::new(|| {
    let vaults = uoe_result!(
        jsonl::load::<Vault>(&paths::VAULTS_FILE_PATH),
        "Failed to load vaults data"
    );

    let mut vault_map: HashMap<u32, Vault> = HashMap::new();
    for v in vaults {
        vault_map.insert(v.id, v);
    }

    vault_map
});

/// 根据vault_id获取vault名字，用于log、list等展示
pub fn get_name(id: u32) -> String {
    let vault = uoe_option!(
        VAULT_MAP.get(&id),
        format!("Vault with id:{} not found", id)
    );
    vault.name.clone()
}

/// 修改现在的 vault
fn use_vault(name: &str) -> Result<(), ArchiverError> {
    let vault = VAULT_MAP.iter().find(|(_, vault)| vault.name == name);
    if vault.is_none() {
        return err_info!("Vault '{}' not found", name);
    }

    // 更新current_vault_id
    let (id, _) = vault.unwrap();
    let mut config = wrap_result!(config::load())?;
    config.current_vault_id = *id;
    wrap_result!(config::save(&config))?;

    Ok(())
}

/// 创建一个新的 vault，不能重名
fn create_vault(name: &str, remark: &Option<String>) -> Result<(), ArchiverError> {
    let vault = VAULT_MAP.iter().find(|(_, vault)| vault.name == name);
    if vault.is_some() {
        return err_info!("Vault with the same name '{}' already exists", name);
    }

    let next_id = auto_incr::vault_id::next();
    let vault = Vault {
        id: next_id,
        name: name.to_string(),
        remark: remark.clone().unwrap_or("".to_string()),
        created_at: dt::now_str(),
    };

    wrap_result!(jsonl::append(&vault, &paths::VAULTS_FILE_PATH))?;

    // 此处不需要VAULTS.push(vault)，因为创建结束后就退出了
    Ok(())
}
