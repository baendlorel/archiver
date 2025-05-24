use crate::{err_info, log_if_err, wrap_err_fatal, wrap_result};

use chrono::Local;
use owo_colors::OwoColorize;
use std::fs;
use std::path::PathBuf;

use crate::misc::{jsonl, paths};
use crate::models::{
    error::ArchiverError,
    types::{CONFIG, ListEntry, ListRow, ListRowColWidth},
};

pub fn handler(all: bool, restored: bool) {
    log_if_err!(print_list(all, restored));
}

pub fn insert(id: u32, target: String, is_dir: bool, dir: String) -> Result<(), ArchiverError> {
    // 自动纳入当前使用的vault
    let vault_id = CONFIG.current_vault_id;
    let archive_entry = ListEntry {
        id,
        vault_id,
        target,
        is_dir,
        dir,
        time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        is_restored: false,
    };

    wrap_result!(jsonl::append(
        &archive_entry,
        paths::LIST_FILE_PATH.as_path()
    ))?;
    Ok(())
}

/// 查找某个id的归档记录，用在restore上
/// - 由于archive_id全局唯一，所以此处需要搜索所有的vault
pub fn find(id: u32) -> Result<(ListEntry, usize, PathBuf), ArchiverError> {
    let list_paths = paths::get_all_list_paths();

    for list_path in list_paths {
        let mut line_index: usize = 0;
        let list = wrap_result!(jsonl::load::<ListEntry>(&list_path))?;
        for entry in list {
            if entry.id == id {
                return Ok((entry, line_index, list_path));
            }
            line_index += 1;
        }
    }

    err_info!("id:{} cannot be found", id)
}

fn print_list(all: bool, restored: bool) -> Result<(), ArchiverError> {
    let list_file_path = paths::LIST_FILE_PATH.as_path();
    if !list_file_path.exists() {
        println!("No archived object yet");
        return Ok(());
    }

    let content = wrap_err_fatal!(fs::read_to_string(list_file_path))?;

    let mut list: Vec<ListRow> = vec![];
    let mut counter = 0;
    for line in content.lines() {
        if line.trim().is_empty() {
            continue; // 跳过空行
        }

        match wrap_err_fatal!(serde_json::from_str::<ListEntry>(line)) {
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
    let field_time = "Archived At";
    let field_id = "ID";
    let field_vault_name = "Vault";
    let field_target = "Item";
    let field_dir = "Directory";

    let mut w = ListRowColWidth {
        time: field_time.len(),
        vault_name: field_vault_name.len(),
        id: field_id.len(),
        target: field_target.len(),
        dir: field_dir.len(),
    };

    for row in list.iter() {
        w.time = w.time.max(row._width.time);
        w.vault_name = w.vault_name.max(row._width.vault_name);
        w.id = w.id.max(row._width.id);
        w.target = w.target.max(row._width.target);
        w.dir = w.dir.max(row._width.dir);
    }

    println!(
        "{}",
        format!(
            "{field_time}{} {field_vault_name}{} {field_id}{} {field_target}{} {field_dir}{}",
            " ".repeat(w.time - field_time.len()),
            " ".repeat(w.vault_name - field_vault_name.len()),
            " ".repeat(w.id - field_id.len()),
            " ".repeat(w.target - field_target.len()),
            " ".repeat(w.dir - field_dir.len()),
            field_time = field_time,
            field_vault_name = field_vault_name,
            field_id = field_id,
            field_target = field_target,
            field_dir = field_dir,
        )
        .bold()
        .underline()
    );

    for row in list.iter() {
        println!(
            "{time}{} {vault_name}{} {id}{} {target}{} {dir}",
            " ".repeat(w.time - row._width.time),
            " ".repeat(w.vault_name - row._width.vault_name),
            " ".repeat(w.id - row._width.id),
            " ".repeat(w.target - row._width.target),
            time = row.time,
            vault_name = row.vault_name,
            id = row.id,
            target = row.target,
            dir = row.dir,
        );
    }

    Ok(())
}
