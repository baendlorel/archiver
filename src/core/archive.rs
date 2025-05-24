use crate::{as_fatal, info, must_some, warn, wrap_result};

use std::{
    ffi::{OsStr, OsString},
    fs,
    path::PathBuf,
};

use super::{auto_incr, list};
use crate::{
    misc::{ForceToString, force_no_loss_string, paths},
    models::{error::ArchiverError, types::ListEntry},
};

pub fn put(target: &str) -> Result<u32, ArchiverError> {
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
    if invalid_target(&target_path) {
        return warn!(
            "Target cannot be the archiver directory, its parent, or its inner object. Got '{}'",
            target
        );
    }

    // 必须无损转换OsString
    let target_dir =
        must_some!(target_path.parent(), "Fail to get target directory").force_to_string();

    let target_name: &OsStr = must_some!(target_path.file_name(), "Fail to get target name");

    // 必须无损转换OsString
    let target_name_str = force_no_loss_string(target_name);

    // 都没有异常，那么开始归档
    let is_dir = target_path.is_dir(); // 不能在rename之后调用，否则目录已经没了，百分百不是
    let next_id = auto_incr::archive_id::next();
    let archived_path = vault_path.join(next_id.to_string());

    as_fatal!(fs::rename(&target_path, archived_path))?;
    wrap_result!(list::insert(next_id, target_name_str, is_dir, target_dir))?;

    Ok(next_id)
}

/// 属于Archiver自己的文件夹，以及其父文件夹，不允许put
fn invalid_target(target_path: &PathBuf) -> bool {
    let root = paths::ROOT_DIR.as_path();

    // 这句可以判定target是不是Archiver及其子目录
    if target_path.starts_with(root) {
        return true;
    }

    // 这句可以判定target是不是Archiver的父目录
    for ancestor in root.ancestors() {
        if target_path == ancestor {
            return true;
        }
    }

    false
}

pub fn restore(id: u32) -> Result<ListEntry, ArchiverError> {
    let (entry, line_index, list_path) = wrap_result!(list::find(id))?;
    if entry.is_restored {
        return info!(
            "id:{} has already been restored to '{}'",
            id,
            entry.get_target_path()
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

    as_fatal!(fs::rename(archive_path, target_path))?;
    wrap_result!(mark_as_restored(line_index, &list_path))?;
    Ok(entry)
}

/// 只会在目标已经被restored之后调用
fn mark_as_restored(line_index: usize, list_path: &std::path::Path) -> Result<(), ArchiverError> {
    // 读取整个文件
    let content = as_fatal!(fs::read_to_string(&list_path))?;

    let mut lines: Vec<&str> = content.lines().collect();
    let target_line = lines[line_index];
    let modified_line = {
        // 把这条记录标记为restored
        let mut entry = as_fatal!(serde_json::from_str::<ListEntry>(target_line))?;
        entry.is_restored = true;
        as_fatal!(serde_json::to_string(&entry))?
    };

    lines[line_index] = modified_line.as_str();

    // 将内容写回文件
    as_fatal!(fs::write(&list_path, lines.join("\n") + "\n"))?;

    Ok(())
}
