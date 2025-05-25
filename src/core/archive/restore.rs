use crate::{as_fatal, info, wrap_result};

use std::{ffi::OsString, fs, path::PathBuf};

use super::list;
use crate::{
    misc::{ForceToString, jsonl, paths},
    models::{error::ArchiverError, types::ListEntry},
};

pub fn restore(id: u32) -> Result<ListEntry, ArchiverError> {
    let (mut list, index) = wrap_result!(list::find_one(id))?;
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
