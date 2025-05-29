use crate::{as_fatal, info, must_some, warn, wrap_result};

use std::{ffi::OsStr, fs, path::PathBuf};

use super::list;
use crate::misc::paths;
use crate::models::{error::ArchiverResult, types::ListEntry};
use crate::traits::ForceToString;

pub fn put(item: &str, message: Option<String>, vault_id: u32) -> ArchiverResult<ListEntry> {
    // 不能trim不能检测为空，否则无法正确处理带空格的文件/文件夹名
    let item_path = as_fatal!(paths::CWD.join(item).canonicalize())?;

    // 目标不存在则报错
    if !item_path.exists() {
        return info!("'{}' does not exist.", item_path.force_to_string());
    }

    if paths::ROOT_DIR.starts_with(&item_path) {
        return warn!(
            "Target cannot be a parent directory of archiver or itself. Got '{}'",
            item
        );
    }

    // & 路径相关性检测：不能归档归档器本身、其父目录、其子目录
    if is_unallowed_path(&item_path) {
        return warn!(
            "Target cannot be the archiver directory, its parent, or its inner object. Got '{}'",
            item
        );
    }

    // 下面这段逻辑是否写在ListEntry::new()里？
    // 必须无损转换OsString
    let item_dir = must_some!(item_path.parent(), "Fail to get item directory").force_to_string();

    let item_name: &OsStr = must_some!(item_path.file_name(), "Fail to get item name");

    // 必须无损转换OsString
    let item_name_str = item_name.force_to_string();

    // * 下面开始归档
    // 准备字段
    let is_dir = item_path.is_dir(); // 不能在rename之后调用，否则目录已经没了，百分百不是
    let message: String = if let Some(m) = message {
        m
    } else {
        String::new()
    };
    let entry = ListEntry::new(item_name_str, is_dir, item_dir, message, vault_id);
    let archived_path = paths::get_archived_path(entry.id, entry.vault_id);

    // 先移动再插表
    as_fatal!(fs::rename(&item_path, archived_path))?;
    wrap_result!(list::insert(&entry))?;

    Ok(entry)
}

/// 目标路径如果满足下列情况之一，不允许put
/// 1. 等于归档器本身
/// 2. 以归档器路径开头的任何子路径
/// 3. 等于归档器的父目录
fn is_unallowed_path(item_path: &PathBuf) -> bool {
    let root = paths::ROOT_DIR.as_path();

    // 判定1、2
    if item_path.starts_with(root) {
        return true;
    }

    // 判定3
    for ancestor in root.ancestors() {
        if item_path == ancestor {
            return true;
        }
    }

    false
}
