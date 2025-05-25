use crate::{as_fatal, info, wrap_result};

use owo_colors::OwoColorize;
use std::fs;

use crate::{
    misc::{jsonl, paths},
    models::{
        error::ArchiverError,
        types::{ListColumnLen, ListEntry, ListRow},
    },
};

/// 将归档记录插入到列表中
/// - 自动生成部分字段
pub fn insert(entry: &ListEntry) -> Result<(), ArchiverError> {
    wrap_result!(jsonl::append(entry, paths::LIST_FILE_PATH.as_path()))?;
    Ok(())
}

/// 查找某个id的归档记录，用在restore上
/// - 由于archive_id全局唯一，所以此处需要搜索所有的vault
pub fn find(id: u32) -> Result<(Vec<ListEntry>, usize), ArchiverError> {
    let list = wrap_result!(jsonl::load::<ListEntry>(paths::LIST_FILE_PATH.as_path()))?;
    let index = list.iter().position(|entry| entry.id == id);
    if let Some(index) = index {
        return Ok((list, index));
    }
    info!("id:{} cannot be found", id)
}

pub fn display(all: bool, restored: bool) -> Result<(), ArchiverError> {
    let list_file_path = paths::LIST_FILE_PATH.as_path();
    if !list_file_path.exists() {
        println!("No archived object yet");
        return Ok(());
    }

    let content = as_fatal!(fs::read_to_string(list_file_path))?;

    let mut list: Vec<ListRow> = vec![];
    let mut counter = 0;
    for line in content.lines() {
        if line.trim().is_empty() {
            continue; // 跳过空行
        }

        match as_fatal!(serde_json::from_str::<ListEntry>(line)) {
            Ok(entry) => {
                // 展示条件为全部展示，或者展示已恢复的，或者展示未恢复的
                // * all || (restored && entry.is_restored) || (!restored && !entry.is_restored)
                let display_condition = all || (restored == entry.is_restored);

                // 设置了all的话，展示全部，否则只展示未恢复的对象
                if display_condition {
                    counter += 1;
                    list.push(entry.to_row());
                }
            }
            Err(e) => e.display(),
        }
    }

    if counter == 0 {
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
        println!("{}", row.to_styled(&col_len));
        // println!(
        //     "{time}{} {vault_name}{} {id}{} {target}{} {dir}",
        //     " ".repeat(col_len.time - cur.time),
        //     " ".repeat(col_len.vault_name - cur.vault_name),
        //     " ".repeat(col_len.id - cur.id),
        //     " ".repeat(col_len.target - cur.target),
        //     time = row.time,
        //     vault_name = row.vault_name,
        //     id = row.id,
        //     target = row.target,
        //     dir = row.dir,
        // );
    }

    Ok(())
}
