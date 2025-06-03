use crate::traits::ForceToString;
use crate::{as_fatal, oper, opt_map, wrap_result};

use std::fs;

use super::{list, sl};
use crate::cli::{Operation, short};
use crate::core::log;
use crate::misc::paths;
use crate::models::error::ArchiverResult;
use crate::models::types::{ListEntry, LogLevel};

// todo put和restore都是函数单条/循环函数的，只有mv是一块的。是不是put也有频繁sl jsonl文件的问题？
// ? put和restore都感觉太重了，应该舍弃批量归纳批量归档

/// 批量移动归档对象到指定的vault_id
///
/// ! 必须在vault_id确认存在时方可调用
pub fn batch_mv(satisfies: impl Fn(&ListEntry) -> bool, vault_id: u32) -> ArchiverResult<usize> {
    let mut list = wrap_result!(list::find_all())?;

    let mut count: usize = 0;
    for entry in list.iter_mut() {
        if !satisfies(&entry) {
            continue;
        }
        count += 1;
        let from = paths::get_archived_path(entry.id, entry.vault_id);
        let to = paths::get_archived_path(entry.id, vault_id);

        // 移动后写日志，移动一条写一条，方便中断后追查
        as_fatal!(fs::rename(&from, &to))?;

        let to = &to.force_to_string();
        let oper = oper!(short::main::MOVE, None, [entry.id], opt_map![to], "sys");

        log::sys(
            oper,
            LogLevel::Success,
            Some(entry.id),
            Some(entry.vault_id),
            format!("{}->{}", entry.vault_id, vault_id),
        );

        entry.vault_id = vault_id;
    }

    wrap_result!(sl::save(&list))?;

    Ok(count)
}
