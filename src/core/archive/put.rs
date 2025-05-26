use crate::{as_fatal, info, must_some, warn, wrap_result};

use std::{ffi::OsStr, fs, path::PathBuf};

use super::list;
use crate::misc::{ForceToString, paths};
use crate::models::{error::ArchiverResult, types::ListEntry};

pub fn put(target: &str, message: &Option<String>) -> ArchiverResult<ListEntry> {
    // 不能trim不能检测为空，否则无法正确处理带空格的文件/文件夹名
    let cwd: std::path::PathBuf = paths::CWD.clone();
    let target_path = as_fatal!(cwd.join(target).canonicalize())?;

    // 目标不存在则报错
    if !target_path.exists() {
        return info!("'{}' does not exist.", target_path.force_to_string());
    }

    if paths::ROOT_DIR.starts_with(&target_path) {
        return warn!(
            "Target cannot be a parent directory of archiver or itself. Got '{}'",
            target
        );
    }

    // & 路径相关性检测：不能归档归档器本身、其父目录、其子目录
    if is_unallowed_path(&target_path) {
        return warn!(
            "Target cannot be the archiver directory, its parent, or its inner object. Got '{}'",
            target
        );
    }

    // 下面这段逻辑是否写在ListEntry::new()里？
    // 必须无损转换OsString
    let target_dir =
        must_some!(target_path.parent(), "Fail to get target directory").force_to_string();

    let target_name: &OsStr = must_some!(target_path.file_name(), "Fail to get target name");

    // 必须无损转换OsString
    let target_name_str = target_name.force_to_string();

    // * 下面开始归档
    // 准备字段
    let is_dir = target_path.is_dir(); // 不能在rename之后调用，否则目录已经没了，百分百不是

    // 新建实例
    let message: String = if let Some(m) = message {
        m.clone()
    } else {
        String::new()
    };
    let entry = ListEntry::new(target_name_str, is_dir, target_dir, message);
    let archived_path = paths::get_archived_path(entry.id, entry.vault_id);

    // 先移动再插表
    as_fatal!(fs::rename(&target_path, archived_path))?;
    wrap_result!(list::insert(&entry))?;

    Ok(entry)
}

/// 目标路径如果满足下列情况之一，不允许put
/// 1. 等于归档器本身
/// 2. 以归档器路径开头的任何子路径
/// 3. 等于归档器的父目录
fn is_unallowed_path(target_path: &PathBuf) -> bool {
    let root = paths::ROOT_DIR.as_path();

    // 判定1、2
    if target_path.starts_with(root) {
        return true;
    }

    // 判定3
    for ancestor in root.ancestors() {
        if target_path == ancestor {
            return true;
        }
    }

    false
}
