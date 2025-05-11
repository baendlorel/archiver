use chrono::Local;
use std::fs;

use crate::misc::{paths, write_entry};
use crate::models::errors::ListError;
use crate::models::types::ArchiveEntry;
use owo_colors::OwoColorize;

pub fn handler() {
    match load() {
        Ok(_) => {}
        Err(e) => println!("{}", e.to_string()),
    }
}

fn save(id: u32, target: String, is_dir: bool, dir: String) -> Result<(), ListError> {
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let archive_entry = ArchiveEntry {
        id,
        target,
        is_dir,
        dir,
        time,
    };
    let list_file_path = paths::list_file_path();
    write_entry(&archive_entry, list_file_path).map_err(|e| ListError::IoError(e.to_string()))?;
    println!("Operation log saved");
    Ok(())
}

fn load() -> Result<(), ListError> {
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

        let result = serde_json::from_str::<ArchiveEntry>(line);
        if let Ok(entry) = &result {
            counter += 1;
            println!("{}", entry.to_str())
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
