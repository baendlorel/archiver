use crate::wrap_result;

use crate::misc::console::table::{Column, ColumnAlign, Table};
use crate::misc::{jsonl, paths};
use crate::models::error::ArchiverResult;
use crate::models::types::ListEntry;

pub fn select(condition: impl Fn(&ListEntry) -> bool) -> ArchiverResult<Vec<ListEntry>> {
    let list = wrap_result!(jsonl::load::<ListEntry>(paths::LIST_FILE_PATH.as_path()))?;
    let list = list.into_iter().filter(|entry| condition(entry)).collect();
    Ok(list)
}

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

pub fn display(all: bool, restored: bool) -> ArchiverResult<()> {
    let list = wrap_result!(select(|entry| all || (restored == entry.is_restored())))?;

    if list.len() == 0 {
        println!("No archived object found");
    }

    // 下面开始输出对好了空格的列表
    Table::display(
        vec![
            Column::with_name("Archived At"),
            Column::with_name("ID"),
            Column::with_name("Item"),
            Column::new("Directory", ColumnAlign::Left, 0),
        ],
        &list,
    );

    Ok(())
}
