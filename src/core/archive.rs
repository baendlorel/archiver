use crate::{as_fatal, info, misc::jsonl, must_some, warn, wrap_result};

use std::{
    ffi::{OsStr, OsString},
    fs,
    path::PathBuf,
};

use super::list;
use crate::{
    misc::{ForceToString, force_no_loss_string, paths},
    models::{error::ArchiverError, types::ListEntry},
};

pub fn put(target: &str) -> Result<ListEntry, ArchiverError> {
    // 不能trim不能检测为空，否则无法正确处理带空格的文件/文件夹名
    let cwd: std::path::PathBuf = paths::CWD.clone();
    let target_path = as_fatal!(cwd.join(target).canonicalize())?;
    let vault_path = paths::get_default_vault_path();

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
    if not_allowed_path(&target_path) {
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
    let target_name_str = force_no_loss_string(target_name);

    // * 下面开始归档
    // 准备字段
    let is_dir = target_path.is_dir(); // 不能在rename之后调用，否则目录已经没了，百分百不是

    // 新建实例
    let entry = ListEntry::new(target_name_str, is_dir, target_dir);
    let archived_path = vault_path.join(entry.id.to_string());

    // 先移动再插表
    as_fatal!(fs::rename(&target_path, archived_path))?;
    wrap_result!(list::insert(&entry))?;

    Ok(entry)
}

/// 目标路径如果满足下列情况之一，不允许put
/// 1. 等于归档器本身
/// 2. 以归档器路径开头的任何子路径
/// 3. 等于归档器的父目录
fn not_allowed_path(target_path: &PathBuf) -> bool {
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

pub fn restore(id: u32) -> Result<ListEntry, ArchiverError> {
    let (mut list, index) = wrap_result!(list::find(id))?;
    let entry = &list[index];
    if entry.is_restored {
        return info!(
            "id:{} has already been restored to '{}'",
            id,
            entry.get_target_path_string()
        );
    }

    let target_name = OsString::from(&entry.target);
    let dir = PathBuf::from(OsString::from(&entry.dir));
    let target_path = dir.join(target_name);
    let archive_path = paths::get_archived_path(id, entry.vault_id);

    // 要检查archive里面的文件和系统外面的路径是否都存在
    // 还要检查复制后是否会导致文件覆盖
    if target_path.exists() {
        return info!(
            "Path '{}' already exists, please remove or rename it first",
            target_path.force_to_string()
        );
    }

    if !archive_path.exists() {
        return info!(
            "The archive file id:{} does not exist",
            archive_path.force_to_string()
        );
    }

    // 先确保上面两个不异常
    // 再确保原目录存在
    if !dir.exists() {
        as_fatal!(fs::create_dir_all(&dir))?;
    }

    // 和put一样，先移动文件，再改表
    as_fatal!(fs::rename(archive_path, target_path))?;
    // 标记为已恢复
    list[index].is_restored = true;
    wrap_result!(jsonl::save(&list, paths::LIST_FILE_PATH.as_path()))?;

    Ok(list[index].clone())
}
