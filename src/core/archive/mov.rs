use crate::{as_fatal, info, warn, wrap_result};

use std::fs;

use super::{list, sl};
use crate::core::vault;
use crate::misc::paths;
use crate::models::error::ArchiverResult;
use crate::traits::CustomColors;

/// move前校验
/// - vault_id必须存在
/// - id必须都存在
/// - 归档记录的vault_id不能和目标vault_id相同
/// - id对应的archiver内文件必须存在
/// - 要移动到的地方不能存在同名的文件/文件夹
pub fn mov_check(ids: &[u32], vault_id: u32) -> ArchiverResult<()> {
    // 检查vault_id是否存在，get_name是must_ok的，所以执行即可
    let vault_name = vault::get_name(vault_id);

    // 检测ids数组
    if ids.is_empty() {
        return info!("No IDs provided for moving.");
    }

    let list = list::find_all()?;
    for id in ids {
        let entry = list.iter().find(|entry| entry.id == *id);
        // id存在
        if entry.is_none() {
            return info!("Id: {} not found in the archive list", id.styled_id());
        }
        let entry = entry.unwrap();

        // 不能已经在目标vault_id中
        if entry.vault_id == vault_id {
            return info!(
                "Id: {} is already in vault: {}",
                id.styled_id(),
                vault::get_name(vault_id)
            );
        }

        // archiver内目录必须存在
        let exists_origin = paths::get_archived_path(entry.id, entry.vault_id).exists();
        if !exists_origin {
            return warn!(
                "Id: {} does not exist in the archive directory",
                id.styled_id()
            );
        }

        // 要移动到的目标位置必须为空
        let exists_target = paths::get_archived_path(entry.id, vault_id).exists();
        if exists_target {
            return warn!(
                "There is already an item recorded as the same id: {} in '{}'",
                id.styled_id(),
                vault_name.styled_vault(),
            );
        }
    }

    Ok(())
}

/// 单个移动，移动一个io一次list，就这么办
///
/// & 在mov_check都满足后调用
pub fn mov(id: u32, vault_id: u32) -> ArchiverResult<()> {
    let mut list = wrap_result!(list::find_all())?;
    let index = list.iter().position(|entry| entry.id == id).unwrap();

    // 校验过了，该有的有，不该有的没有
    let from = paths::get_archived_path(list[index].id, list[index].vault_id);
    let to = paths::get_archived_path(list[index].id, vault_id);

    // 动手
    as_fatal!(fs::rename(&from, &to))?;
    list[index].vault_id = vault_id;
    wrap_result!(sl::save(&list))?;
    Ok(())
}
