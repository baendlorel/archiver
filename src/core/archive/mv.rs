use crate::{as_fatal, map, wrap_result};

use crate::cli::short;
use crate::misc::paths;

use std::fs;

use super::sl;
use crate::core::log;
use crate::models::error::ArchiverResult;
use crate::models::types::{ListEntry, Operation};

// todo 这样会不会有点奇怪？put和restore都是单条但循环处理的，只有mv是一块的。是不是put也有频繁sl jsonl文件的问题？
/// 批量移动归档对象到指定的vault_id
///
/// ! 必须在vault_id确认存在时方可调用
pub fn batch_mv(satisfies: impl Fn(&ListEntry) -> bool, vault_id: u32) -> ArchiverResult<usize> {
    let mut list = wrap_result!(sl::load())?;

    let mut args: Vec<String> = vec![];
    let mut count: usize = 0;
    for entry in list.iter_mut() {
        if !satisfies(&entry) {
            continue;
        }
        count += 1;
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
            map!["to".to_string() => vault_id.to_string()],
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

    Ok(count)
}
