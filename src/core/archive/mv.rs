use crate::{as_fatal, map, wrap_result};

use crate::cli::short;
use crate::misc::paths;

use std::fs;

use super::sl;
use crate::core::log;
use crate::models::error::ArchiverResult;
use crate::models::types::{ListEntry, Operation};

// todo 所有多重输入，为了反复根据id查询，可能应该把sl::load改为返回HashMap<u32, ListEntry>，同时配套save入参

/// 批量移动归档对象到指定的vault_id
///
/// ! 必须在vault_id确认存在时方可调用
pub fn batch_mv(satisfies: impl Fn(&ListEntry) -> bool, vault_id: u32) -> ArchiverResult<()> {
    let mut list = wrap_result!(sl::load())?;

    let mut args: Vec<String> = vec![];
    for entry in list.iter_mut() {
        if !satisfies(&entry) {
            continue;
        }
        args.push(entry.id.to_string());
        let from = paths::get_archived_path(entry.id, entry.vault_id);
        let to = paths::get_archived_path(entry.id, vault_id);

        // 移动后写日志，移动一条写一条，方便中断后追查
        as_fatal!(fs::rename(from, to))?;
        let oper = Operation::system(
            short::main::MOVE,
            "",
            "",
            args,
            map!["to".to_string() => vault_id],
        );

        log::save_system_oper(
            &oper,
            true,
            Some(entry.id),
            Some(entry.vault_id),
            format!("{} -> {}", entry.vault_id, vault_id),
        );

        entry.vault_id = vault_id;
    }

    wrap_result!(sl::save(&list))?;

    Ok(())
}
