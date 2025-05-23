use crate::{err_info, err_warn, wrap_result};

use std::path;

use crate::{
    misc::{ForceToString, paths},
    models::{error::ArchiverError, types::AliasEntry},
};

use super::sl;

// # 业务函数
pub fn remove_alias(alias_entry: &str) -> Result<(), ArchiverError> {
    let (alias, origin) = wrap_result!(parse_alias_entry_string(alias_entry))?;
    let mut config = sl::load()?;

    let target_index = config
        .alias
        .iter()
        .position(|entry| entry.alias == alias && entry.origin == origin);

    if let Some(index) = target_index {
        config.alias.remove(index);
        wrap_result!(sl::save(&config))?;
    } else {
        return err_info!("Alias '{}' with origin '{}' not found", alias, origin);
    }

    Ok(())
}

pub fn set_alias(alias_entry: &str) -> Result<(), ArchiverError> {
    let (alias, origin) = wrap_result!(parse_alias_entry_string(alias_entry))?;
    let mut config = wrap_result!(sl::load())?;

    for entry in &config.alias {
        if entry.alias == alias {
            return err_info!(
                "Alias '{}' is already bound with origin '{}'",
                entry.alias,
                entry.origin
            );
        }
        if entry.origin == origin {
            return err_info!(
                "Origin '{}' is already bound with alias '{}'",
                entry.origin,
                entry.alias
            );
        }
    }

    config.alias.push(AliasEntry {
        alias: alias.to_string(),
        origin: origin.to_string(),
    });
    wrap_result!(sl::save(&config))?;

    Ok(())
}

// # 辅助函数
pub fn parse_alias_entry_string(alias_entry: &str) -> Result<(String, String), ArchiverError> {
    if let Some((alias, origin)) = alias_entry.split_once("=") {
        if alias.is_empty() {
            return err_warn!("alias is empty. Got '{}'", alias_entry);
        }

        if origin.is_empty() {
            return err_warn!("origin is empty. Got '{}'", alias_entry);
        }

        // 去掉origin后面的斜杠
        let alias = alias.trim_end_matches(path::MAIN_SEPARATOR);
        let origin = origin.trim_end_matches(path::MAIN_SEPARATOR);

        if alias == origin {
            return err_warn!("Alias and origin cannot be the same. Got '{}'", alias_entry);
        }

        if origin == paths::HOME_DIR.force_to_string() || alias == "~" {
            return err_info!("HOME_DIR already has a default alias '~', no need to set it again.");
        }
        return Ok((alias.to_string(), origin.to_string()));
    } else {
        err_warn!(
            "Alias config string must take the form of 'xxx=/a/b'. Got '{}'",
            alias_entry
        )
    }
}
