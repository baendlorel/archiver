use crate::models::json_serde::JsonSerde;
use crate::{err_info, log_if_err, wrap_err_fatal, wrap_result};

use chrono::Local;
use owo_colors::OwoColorize;
use std::fs;

use crate::misc::{append_entry, paths};
use crate::models::{
    error::ArchiverError,
    types::{ListEntry, ListRow, ListRowColWidth},
};

pub fn handler(all: bool, restored: bool) {
    log_if_err!(print_list(all, restored));
}

pub fn insert(id: u32, target: String, is_dir: bool, dir: String) -> Result<(), ArchiverError> {
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let archive_entry = ListEntry {
        id,
        target,
        is_dir,
        dir,
        time,
        is_restored: false,
    };

    wrap_result!(append_entry(
        &archive_entry,
        paths::LIST_FILE_PATH.as_path()
    ))?;
    Ok(())
}

/// 查找某个id的归档记录，用在restore上
pub fn find(id: u32, target_line_index: &mut u32) -> Result<ListEntry, ArchiverError> {
    let content = wrap_err_fatal!(fs::read_to_string(paths::LIST_FILE_PATH.as_path()))?;

    for line in content.lines() {
        *target_line_index += 1;

        if line.trim().is_empty() {
            continue; // 跳过空行
        }

        if let Ok(entry) = ListEntry::from_json_string(line) {
            if entry.id == id {
                // 因为第一行加得太早，这里得减去多加了的
                *target_line_index -= 1;
                return Ok(entry);
            }
        }
    }

    err_info!("id:{} cannot be found", id)
}

/// 只会在目标已经被restored之后调用
pub fn mark_as_restored(target_line_index: u32) -> Result<(), ArchiverError> {
    let list_file_path = paths::LIST_FILE_PATH.as_path();
    // 读取整个文件
    let content = wrap_err_fatal!(fs::read_to_string(&list_file_path))?;

    let mut lines: Vec<&str> = content.lines().collect();
    let target_line = lines[target_line_index as usize];
    let modified_line = {
        // 把这条记录标记为restored
        let mut entry = wrap_err_fatal!(serde_json::from_str::<ListEntry>(target_line))?;
        entry.is_restored = true;
        wrap_err_fatal!(serde_json::to_string(&entry))?
    };

    lines[target_line_index as usize] = modified_line.as_str();

    // 将内容写回文件
    wrap_err_fatal!(fs::write(&list_file_path, lines.join("\n") + "\n"))?;

    Ok(())
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
    let field_target = "Item";
    let field_dir = "Directory";

    let mut max_width = ListRowColWidth {
        time: field_time.len(),
        id: field_id.len(),
        target: field_target.len(),
        dir: field_dir.len(),
    };

    for row in list.iter() {
        max_width.time = max_width.time.max(row._width.time);
        max_width.id = max_width.id.max(row._width.id);
        max_width.target = max_width.target.max(row._width.target);
        max_width.dir = max_width.dir.max(row._width.dir);
    }

    println!(
        "{}",
        format!(
            "{field_time}{} {field_id}{} {field_target}{} {field_dir}{}",
            " ".repeat(max_width.time - field_time.len()),
            " ".repeat(max_width.id - field_id.len()),
            " ".repeat(max_width.target - field_target.len()),
            " ".repeat(max_width.dir - field_dir.len()),
            field_time = field_time,
            field_id = field_id,
            field_target = field_target,
            field_dir = field_dir,
        )
        .bold()
        .underline()
    );

    for row in list.iter() {
        println!(
            "{time}{} {id}{} {target}{} {dir}",
            " ".repeat(max_width.time - row._width.time),
            " ".repeat(max_width.id - row._width.id),
            " ".repeat(max_width.target - row._width.target),
            time = row.time,
            id = row.id,
            target = row.target,
            dir = row.dir,
        );
    }

    Ok(())
}
