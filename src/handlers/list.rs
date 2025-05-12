use chrono::Local;
use std::fs;

use crate::misc::{paths, write_entry};
use crate::models::errors::ListError;
use crate::models::types::ListEntry;
use owo_colors::OwoColorize;

pub fn handler(all: bool) {
    match load(all) {
        Ok(_) => {}
        Err(e) => println!("{}", e.to_string()),
    }
}

pub fn insert(id: u32, target: String, is_dir: bool, dir: String) -> Result<(), ListError> {
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let archive_entry = ListEntry {
        id,
        target,
        is_dir,
        dir,
        time,
        is_restored: false,
    };
    let list_file_path = paths::list_file_path();
    write_entry(&archive_entry, list_file_path).map_err(|e| ListError::IoError(e.to_string()))?;
    println!("Archived file listed");
    Ok(())
}

pub fn find(id: u32, target_line_index: &mut u32) -> Result<ListEntry, ListError> {
    let list_file_path = paths::list_file_path();
    let content = fs::read_to_string(list_file_path)?;

    for line in content.lines() {
        *target_line_index += 1;

        if line.trim().is_empty() {
            continue; // 跳过空行
        }

        let result = serde_json::from_str::<ListEntry>(line);
        if let Ok(entry) = &result {
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

    Err(ListError::TargetNotFound(format!(
        "ID {} not found in the list",
        id
    )))
}

/// Will only be called when the file is successfully restored
pub fn mark_as_restored(target_line_index: u32) -> Result<(), ListError> {
    let list_file_path = paths::list_file_path();
    // 读取整个文件
    let content = fs::read_to_string(&list_file_path)?;

    let mut lines: Vec<&str> = content.lines().collect();
    let target_line = lines[target_line_index as usize];
    let modified_line = {
        let mut entry = serde_json::from_str::<ListEntry>(target_line)?;
        entry.is_restored = true;
        serde_json::to_string(&entry)?
    };

    lines[target_line_index as usize] = modified_line.as_str();

    // 将内容写回文件
    fs::write(&list_file_path, lines.join("\n") + "\n")?;

    Ok(())
}

fn load(all: bool) -> Result<(), ListError> {
    let list_file_path = paths::list_file_path();
    if !list_file_path.exists() {
        println!("No archived object yet");
        return Ok(());
    }

    let content = fs::read_to_string(list_file_path)?;

    let mut counter = 0;
    for line in content.lines() {
        if line.trim().is_empty() {
            continue; // 跳过空行
        }

        let result = serde_json::from_str::<ListEntry>(line);
        if let Ok(entry) = &result {
            counter += 1;
            // 设置了all的话，展示全部，否则只展示未恢复的对象
            if all || !entry.is_restored {
                println!("{}", entry.to_string());
            }
        }

        if let Err(e) = &result {
            println!(
                "{}: {}",
                "Parse list file failed".red(),
                e.to_string().yellow()
            );
            continue;
        }
    }

    if counter == 0 {
        println!("No archived object found");
    }

    Ok(())
}
