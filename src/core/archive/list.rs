use crate::{info, wrap_result};

use owo_colors::OwoColorize;

use super::sl;
use crate::misc::{jsonl, paths};
use crate::models::error::ArchiverResult;
use crate::models::types::{ListColumnLen, ListEntry, ListRow};

/// 将归档记录插入到列表中
/// - 自动生成部分字段
pub fn insert(entry: &ListEntry) -> ArchiverResult<()> {
    wrap_result!(jsonl::append(entry, paths::LIST_FILE_PATH.as_path()))?;
    Ok(())
}

/// 查找某个id的归档记录，用在restore上
/// - 由于archive_id全局唯一，所以此处需要搜索所有的vault
///
/// 返回ListEntry列表和找到的index
pub fn find_one(id: u32) -> ArchiverResult<(Vec<ListEntry>, usize)> {
    let list = wrap_result!(sl::load())?;
    let index = list.iter().position(|entry| entry.id == id);
    if let Some(index) = index {
        return Ok((list, index));
    }
    info!("id: {} cannot be found", id)
}

pub fn find(ids: &[u32]) -> ArchiverResult<Vec<ListEntry>> {
    let list = wrap_result!(sl::load())?;

    let result = list
        .into_iter()
        .filter(|entry| ids.contains(&entry.id))
        .collect();

    Ok(result)
}

pub fn display(all: bool, restored: bool) -> ArchiverResult<()> {
    let list = wrap_result!(sl::load())?;
    let list = list
        .iter()
        .filter(|entry| all || (restored == entry.is_restored()))
        .map(|entry| entry.to_row())
        .collect::<Vec<ListRow>>();

    if list.len() == 0 {
        println!("No archived object found");
    }

    // 下面开始输出对好了空格的列表
    // 字段名称
    let field_archived_at = "Archived At";
    let field_id = "ID";
    let field_vault_name = "Vault";
    let field_target = "Item";
    let field_dir = "Directory";

    let mut col_len = ListColumnLen {
        archived_at: field_archived_at.len(),
        vault_name: field_vault_name.len(),
        id: field_id.len(),
        target: field_target.len(),
        dir: field_dir.len(),
    };

    for row in list.iter() {
        let cur = row.get_len();
        col_len.archived_at = col_len.archived_at.max(cur.archived_at);
        col_len.vault_name = col_len.vault_name.max(cur.vault_name);
        col_len.id = col_len.id.max(cur.id);
        col_len.target = col_len.target.max(cur.target);
        col_len.dir = col_len.dir.max(cur.dir);
    }

    println!(
        "{}",
        format!(
            "{field_archived_at}{} {field_vault_name}{} {field_id}{} {field_target}{} {field_dir}{}",
            " ".repeat(col_len.archived_at - field_archived_at.len()),
            " ".repeat(col_len.vault_name - field_vault_name.len()),
            " ".repeat(col_len.id - field_id.len()),
            " ".repeat(col_len.target - field_target.len()),
            " ".repeat(col_len.dir - field_dir.len()),
            field_archived_at = field_archived_at,
            field_vault_name = field_vault_name,
            field_id = field_id,
            field_target = field_target,
            field_dir = field_dir,
        )
        .bold()
        .underline()
    );

    for row in list.iter() {
        println!("{}", row.to_display(&col_len));
    }

    Ok(())
}
