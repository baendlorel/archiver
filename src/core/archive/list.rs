use crate::wrap_result;

use owo_colors::OwoColorize;

use crate::misc::{jsonl, paths};
use crate::models::error::ArchiverResult;
use crate::models::types::{ListColumnLen, ListEntry, ListRow};
use crate::traits::StripAnsi;

// pub fn select(condition: impl Fn(&ListEntry) -> bool) -> ArchiverResult<Vec<ListEntry>> {
//     let list = wrap_result!(jsonl::load::<ListEntry>(paths::LIST_FILE_PATH.as_path()))?;
//     let list = list.into_iter().filter(|entry| condition(entry)).collect();
//     Ok(list)
// }

pub fn select_all() -> ArchiverResult<Vec<ListEntry>> {
    let list = wrap_result!(jsonl::load::<ListEntry>(paths::LIST_FILE_PATH.as_path()))?;
    Ok(list)
}

/// 将归档记录插入到列表中
/// - 自动生成部分字段
pub fn insert(entry: &ListEntry) -> ArchiverResult<()> {
    wrap_result!(jsonl::append(entry, paths::LIST_FILE_PATH.as_path()))?;
    Ok(())
}

// todo 利用stripansi的trait改造这里，最好能通用
pub fn display(all: bool, restored: bool) -> ArchiverResult<()> {
    let list = wrap_result!(select_all())?;

    let rows = list
        .iter()
        .filter(|entry| all || (restored == entry.is_restored()))
        .map(|entry| entry.to_row())
        .collect::<Vec<ListRow>>();

    if rows.len() == 0 {
        println!("No archived object found");
    }

    // 下面开始输出对好了空格的列表
    // 字段名称
    let field_archived_at = "Archived At";
    let field_id = "ID";
    let field_item = "Item";
    let field_dir = "Directory";

    let mut col_len = ListColumnLen {
        archived_at: field_archived_at.len(),
        id: field_id.len(),
        item: field_item.len(),
        dir: field_dir.len(),
    };

    for row in rows.iter() {
        let cur = row.get_len();
        col_len.archived_at = col_len.archived_at.max(cur.archived_at);
        col_len.id = col_len.id.max(cur.id);
        col_len.item = col_len.item.max(cur.item);
        col_len.dir = col_len.dir.max(cur.dir);
    }

    println!(
        "{}",
        format!(
            "{field_archived_at}{} {field_id}{} {field_item}{} {field_dir}{}",
            " ".repeat(col_len.archived_at - field_archived_at.true_len()),
            " ".repeat(col_len.id - field_id.true_len()),
            " ".repeat(col_len.item - field_item.true_len()),
            " ".repeat(col_len.dir - field_dir.true_len()),
            field_archived_at = field_archived_at,
            field_id = field_id,
            field_item = field_item,
            field_dir = field_dir,
        )
        .bold()
        .underline()
    );

    for row in rows.iter() {
        println!("{}", row.to_display(&col_len));
    }

    Ok(())
}
