use crate::{info, warn, wrap_result};

use std::path;

use super::sl;
use crate::misc::{console, paths};
use crate::models::error::ArchiverResult;
use crate::traits::ForceToString;

// # 业务函数
pub fn remove_alias(alias_entry: &str) -> ArchiverResult<()> {
    let (alias, origin) = wrap_result!(parse(alias_entry))?;
    let mut config = wrap_result!(sl::load())?;

    if let Some(value) = config.alias_map.get(&alias) {
        // 如果origin解析出来是空的，那么允许它删除
        // 如果不是空的，那么需要检查是否和保存的一致
        if !origin.is_empty() && value != &origin {
            return info!(
                "Alias '{}' is bound to '{}', not '{}'. Use `arv config alias remove {}={}` to remove it.",
                alias, value, origin, alias, value
            );
        }
    } else {
        return info!("Found no alias named '{}'.", alias);
    }

    config.alias_map.remove(&alias);
    wrap_result!(sl::save(&config))?;

    Ok(())
}

pub fn set_alias(alias_entry: &str) -> ArchiverResult<()> {
    let (alias, origin) = wrap_result!(parse(alias_entry))?;
    let mut config = wrap_result!(sl::load())?;

    // 检查是否已经存在相同的别名或原始路径
    let value = config.alias_map.get(&alias);
    if let Some(value) = value {
        if value == &origin {
            return info!(
                "Alias '{}' is already bound to origin '{}'. No need to set it again.",
                alias, origin
            );
        } else {
            println!("Alias '{}' is already bound to '{}'.", alias, value);
            if !console::confirm("Would you like to override it?") {
                println!("Alias config cancelld.");
                return info!(
                    "Alias '{}={}' exists. User chose not to override.",
                    alias, value
                );
            }
        }
    }

    config.alias_map.insert(alias, origin);
    wrap_result!(sl::save(&config))?;
    Ok(())
}

// # 辅助函数

/// 解析用户输入的alias_entry，用于后续的增删
/// - 由于是required的参数，所以不可能为空
pub fn parse(alias_entry: &str) -> ArchiverResult<(String, String)> {
    if let Some((alias, origin)) = alias_entry.split_once("=") {
        // & 这里不需要origin解析出来非得不是空值，空值可以匹配任意值，方便remove
        // if origin.is_empty() {
        //     return warn!("origin is empty. Got '{}'", alias_entry);
        // }

        // 去掉origin后面的斜杠
        let alias = alias.trim_end_matches(path::MAIN_SEPARATOR);
        let origin = origin.trim_end_matches(path::MAIN_SEPARATOR);

        if alias == origin {
            return warn!("Alias and origin cannot be the same. Got '{}'", alias_entry);
        }

        if origin == paths::HOME_DIR.force_to_string() || alias == "~" {
            return info!("HOME_DIR already has a default alias '~', no need to set it again.");
        }
        return Ok((alias.to_string(), origin.to_string()));
    } else {
        // 如果为None，则视作只输入了alias
        return Ok((alias_entry.to_string(), String::new()));
    }
}
