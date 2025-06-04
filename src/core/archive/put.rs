use crate::{as_fatal, info, must_some, warn, wrap_result};

use std::collections::HashSet;
use std::{fs, path::PathBuf};

use super::list;
use crate::misc::paths;
use crate::models::{error::ArchiverResult, types::ListEntry};
use crate::traits::ForceToString;

/// 检查put的路径数组是否合法
/// - unallowed不行
/// - 重复的不行
pub fn put_check(items: &[String]) -> ArchiverResult<()> {
    let mut set: HashSet<PathBuf> = HashSet::new();
    // 检查每个路径是否存在
    for item in items {
        let item_path = as_fatal!(paths::CWD.join(item).canonicalize())?;

        if !item_path.exists() {
            return info!("Non-exist path detected: '{}'", item_path.force_to_string());
        }

        if is_unallowed_path(&item_path) {
            return warn!(
                "Target cannot be the archiver directory, its parent, or its inner object. Got '{}'",
                item
            );
        }

        if set.contains(&item_path) {
            return warn!(
                "Duplicate path detected: '{}'. Please ensure all paths are unique.",
                item_path.force_to_string()
            );
        }

        set.insert(item_path);
    }

    Ok(())
}

pub fn put(item: &str, message: Option<String>, vault_id: u32) -> ArchiverResult<ListEntry> {
    // & 在put_check之后再调用函数，这里的path是已经校验过的
    let item_path = as_fatal!(paths::CWD.join(item).canonicalize())?;

    // 下面这段逻辑是否写在ListEntry::new()里？
    let item_dir = must_some!(item_path.parent(), "Fail to get item directory").force_to_string();
    let item_name = must_some!(item_path.file_name(), "Fail to get item name").force_to_string();

    // * 下面开始归档
    let is_dir = item_path.is_dir(); // 不能在rename之后调用，否则目录已经没了，百分百不是
    let message = message.unwrap_or(String::new());
    let entry = ListEntry::new(item_name, is_dir, item_dir, message, vault_id);
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
