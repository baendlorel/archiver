use crate::{err, wrap_result};

use crate::core::vault;
use crate::misc::console::table::{Column, Table};
use crate::misc::{jsonl, paths};
use crate::models::error::ArchiverResult;
use crate::models::types::ListEntry;

pub fn find(condition: impl Fn(&ListEntry) -> bool) -> ArchiverResult<Vec<ListEntry>> {
    let list = wrap_result!(jsonl::load::<ListEntry>(paths::LIST_FILE_PATH.as_path()))?;
    let list = list.into_iter().filter(|entry| condition(entry)).collect();
    Ok(list)
}

pub fn find_all() -> ArchiverResult<Vec<ListEntry>> {
    let list = wrap_result!(jsonl::load::<ListEntry>(paths::LIST_FILE_PATH.as_path()))?;
    Ok(list)
}

/// 将归档记录插入到列表中
/// - 自动生成部分字段
pub fn insert(entry: &ListEntry) -> ArchiverResult<()> {
    wrap_result!(jsonl::append(entry, paths::LIST_FILE_PATH.as_path()))?;
    Ok(())
}

pub fn display(all: bool, restored: bool, vault: &Option<String>) -> ArchiverResult<()> {
    let list = match vault {
        Some(v) => {
            let vault_id = vault::get_id(v);
            if vault_id.is_none() {
                return err!("Vault '{}' not found", v);
            }
            let vault_id = vault_id.unwrap();
            wrap_result!(find(
                |entry| entry.vault_id == vault_id && (all || (restored == entry.is_restored()))
            ))?
        }
        None => wrap_result!(find(|entry| all || (restored == entry.is_restored())))?,
    };

    if list.len() == 0 {
        println!("No archived object found");
    }

    // 下面开始输出对好了空格的列表
    Table::display(&list);

    Ok(())
}
