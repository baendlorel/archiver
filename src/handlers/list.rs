use crate::{err_info, println_err, wrap_err, wrap_result};

use chrono::Local;
use owo_colors::OwoColorize;
use std::fs;

use crate::misc::{append_entry, paths};
use crate::models::{
    error::ArchiverError,
    types::{LIST_ROW_FIELD, ListEntry, ListRow, ListRowColWidth},
};

pub fn handler(all: bool) {
    if let Err(e) = print_list(all) {
        println_err!(e);
    }
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

    wrap_result!(append_entry(&archive_entry, paths::LIST_FILE_PATH.clone()))?;
    Ok(())
}

pub fn find(id: u32, target_line_index: &mut u32) -> Result<ListEntry, ArchiverError> {
    let content = wrap_err!(fs::read_to_string(paths::LIST_FILE_PATH.clone()))?;

    for line in content.lines() {
        *target_line_index += 1;

        if line.trim().is_empty() {
            continue; // 跳过空行
        }

        if let Ok(entry) = &serde_json::from_str::<ListEntry>(line) {
            if entry.id == id {
                // 因为第一行加得太早，这里得减去多加了的
                *target_line_index -= 1;
                return Ok(ListEntry {
                    id: entry.id,
                    target: entry.target.clone(),
                    is_dir: entry.is_dir,
                    dir: entry.dir.clone(),
                    time: entry.time.clone(),
                    is_restored: entry.is_restored,
                });
            }
        }
    }

    Err(err_info!(format!("id:{} cannot be found in the list", id)))
}

/// Will only be called when the file is successfully restored
pub fn mark_as_restored(target_line_index: u32) -> Result<(), ArchiverError> {
    let list_file_path = paths::LIST_FILE_PATH.clone();
    // 读取整个文件
    let content = wrap_err!(fs::read_to_string(&list_file_path))?;

    let mut lines: Vec<&str> = content.lines().collect();
    let target_line = lines[target_line_index as usize];
    let modified_line = {
        // 把这条记录标记为restored
        let mut entry = wrap_err!(serde_json::from_str::<ListEntry>(target_line))?;
        entry.is_restored = true;
        wrap_err!(serde_json::to_string(&entry))?
    };

    lines[target_line_index as usize] = modified_line.as_str();

    // 将内容写回文件
    wrap_err!(fs::write(&list_file_path, lines.join("\n") + "\n"))?;

    Ok(())
}

fn print_list(all: bool) -> Result<(), ArchiverError> {
    let list_file_path = paths::LIST_FILE_PATH.clone();
    if !list_file_path.exists() {
        println!("No archived object yet");
        return Ok(());
    }

    let content = wrap_err!(fs::read_to_string(list_file_path))?;

    let mut list: Vec<ListRow> = vec![];
    let mut counter = 0;
    for line in content.lines() {
        if line.trim().is_empty() {
            continue; // 跳过空行
        }

        let result = wrap_err!(serde_json::from_str::<ListEntry>(line));
        if let Ok(entry) = &result {
            // 设置了all的话，展示全部，否则只展示未恢复的对象
            if all || !entry.is_restored {
                counter += 1;
                let row = entry.to_row();
                list.push(row);
            }
        }

        if let Err(e) = &result {
            // println!(
            //     "{}: {}",
            //     "Parse list file failed".red(),
            //     e.to_string().yellow()
            // );
            println_err!(e);
            continue;
        }
    }

    if counter == 0 {
        println!("No archived object found");
    }

    // 下面开始输出对好了空格的列表
    let mut max_width = ListRowColWidth {
        time: LIST_ROW_FIELD.time.len(),
        id: LIST_ROW_FIELD.id.len(),
        target: LIST_ROW_FIELD.target.len(),
        dir: LIST_ROW_FIELD.dir.len(),
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
            " ".repeat(max_width.time - LIST_ROW_FIELD.time.len()),
            " ".repeat(max_width.id - LIST_ROW_FIELD.id.len()),
            " ".repeat(max_width.target - LIST_ROW_FIELD.target.len()),
            " ".repeat(max_width.dir - LIST_ROW_FIELD.dir.len()),
            field_time = LIST_ROW_FIELD.time,
            field_id = LIST_ROW_FIELD.id,
            field_target = LIST_ROW_FIELD.target,
            field_dir = LIST_ROW_FIELD.dir,
        )
        .bold()
        .underline()
    );

    for row in list.iter().clone() {
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
