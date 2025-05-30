use crate::{info, warn, wrap_result};

use std::path::MAIN_SEPARATOR;

use super::{CONFIG, save};
use crate::misc::{console, paths};
use crate::models::error::ArchiverResult;
use crate::traits::ForceToString;

// # 业务函数
pub fn remove(alias_entry: &str) -> ArchiverResult<()> {
    let (alias, origin) = wrap_result!(parse(alias_entry))?;
    let mut config = CONFIG.clone();

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
    wrap_result!(save(&config))?;
    Ok(())
}

pub fn add(alias_entry: &str) -> ArchiverResult<()> {
    let (alias, origin) = wrap_result!(parse(alias_entry))?;
    let mut config = CONFIG.clone();

    if origin.is_empty() {
        return warn!("No origin path detected in alias entry '{}'.", alias_entry);
    }

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
    wrap_result!(save(&config))?;
    Ok(())
}

// # 辅助函数

/// 解析用户输入的alias_entry，用于后续的增删
/// - 由于是required的参数，所以不可能为空
fn parse(alias_entry: &str) -> ArchiverResult<(String, String)> {
    if let Some((alias, origin)) = alias_entry.split_once("=") {
        // & 这里不需要origin解析出来非得不是空值，空值可以匹配任意值，方便remove
        // if origin.is_empty() {
        //     return warn!("origin is empty. Got '{}'", alias_entry);
        // }

        // 去掉origin后面的斜杠
        let alias = alias.trim_end_matches(MAIN_SEPARATOR);
        let origin = origin.trim_end_matches(MAIN_SEPARATOR);

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

/// 将alias应用到一串路径字符串里
pub fn apply(path_str: &str) -> String {
    for (alias, origin) in CONFIG.alias_map.iter() {
        let bytes = path_str.as_bytes();
        let origin_bytes = origin.as_bytes();
        let len = path_str.len();
        let olen = origin.len();

        let mut i = 0;
        while i + olen <= len {
            if &bytes[i..i + olen] == origin_bytes {
                // 判断是不是单词起始
                let is_word_start =
                    i == 0 || !bytes[i - 1].is_ascii_alphanumeric() && bytes[i - 1] != b'_';
                if is_word_start {
                    // 找到单词边界
                    let mut left = i;
                    let mut right = i + olen;
                    // 向左扩展
                    while left > 0
                        && (bytes[left - 1].is_ascii_alphanumeric() || bytes[left - 1] == b'_')
                    {
                        left -= 1;
                    }
                    // 向右扩展
                    while right < len
                        && (bytes[right].is_ascii_alphanumeric() || bytes[right] == b'_')
                    {
                        right += 1;
                    }
                    // 检查单词是否包含 MAIN_SEPARATOR
                    let word = &path_str[left..right];
                    if word.contains(MAIN_SEPARATOR) {
                        let mut replaced = path_str.to_string();
                        replaced.replace_range(i..i + olen, alias);
                        return replaced;
                    }
                }
            }
            i += 1;
        }
    }
    path_str.to_string()
}
