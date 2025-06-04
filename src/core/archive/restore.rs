use crate::{as_fatal, info, wrap_result};

use std::{ffi::OsString, fs, path::PathBuf};

use super::list;
use crate::misc::{jsonl, paths};
use crate::models::error::ArchiverResult;
use crate::models::types::{ListEntry, ListStatus};
use crate::traits::{CustomColors, ForceToString};

/// 校验用于restore的id列表
/// - id必须存在
/// - 状态必须是`ListStatus::Archived`
pub fn restore_check(ids: &[u32]) -> ArchiverResult<()> {
    if ids.is_empty() {
        return info!("No ids provided for restoration");
    }

    let list = wrap_result!(list::find_all())?;
    let mut set: std::collections::HashSet<u32> = std::collections::HashSet::new();
    for &id in ids {
        let entry = list.iter().find(|entry| entry.id == id);
        if entry.is_none() {
            return info!("id: {} cannot be found in the archive list", id.styled_id());
        }
        let entry = entry.unwrap();
        if matches!(entry.status, ListStatus::Archived) {
            return info!(
                "id: {} has already been restored to '{}'",
                id.styled_id(),
                entry.get_item_path_string()
            );
        }
        if !set.insert(id) {
            return info!("Duplicate id detected: {}", id.styled_id());
        }
    }

    Ok(())
}

pub fn restore(id: u32) -> ArchiverResult<ListEntry> {
    let mut list = wrap_result!(list::find_all())?;
    // & ids数组是经过校验的，一定会找到，而且没有restore过
    let index = list.iter().position(|entry| entry.id == id).unwrap();
    let entry = &list[index];

    let item_name = OsString::from(&entry.item);
    let dir = PathBuf::from(OsString::from(&entry.dir));
    let item_path = dir.join(item_name);
    let archive_path = paths::get_archived_path(id, entry.vault_id);

    // 要检查archive里面的文件和系统外面的路径是否都存在
    // 还要检查复制后是否会导致文件覆盖
    if item_path.exists() {
        return info!(
            "Path '{}' already exists, please remove or rename it first",
            item_path.force_to_string()
        );
    }

    if !archive_path.exists() {
        return info!(
            "The archive file id: {} does not exist",
            archive_path.force_to_string()
        );
    }

    // 先确保上面两个不异常
    // 再确保原目录存在
    if !dir.exists() {
        as_fatal!(fs::create_dir_all(&dir))?;
    }

    // 和put一样，先移动文件，再改表
    as_fatal!(fs::rename(archive_path, item_path))?;
    // 标记为已恢复
    list[index].status = ListStatus::Restored;
    wrap_result!(jsonl::save(&list, paths::LIST_FILE_PATH.as_path()))?;

    Ok(list[index].clone())
}
