use owo_colors::OwoColorize;
use std::fs;
use std::path::PathBuf;

use super::{list, log};
use crate::misc::paths;
use crate::models::{errors::RestoreError, types::OperType};

pub fn handler(id: u32) {
    println!("反归档 {}", id.green());
    match restore(id) {
        Ok(_) => println!("{} is successfully restored", id),
        Err(e) => {
            let _ = log::save(OperType::Restore, id.to_string(), false, Some(id));
            println!("{}", e.to_string());
        }
    }
}

fn restore(id: u32) -> Result<(), RestoreError> {
    match list::find(id) {
        Ok(entry) => {
            let dir = PathBuf::from(entry.dir);
            let target_path = dir.join(entry.target.clone());
            let archive_path = paths::root_dir().join(id.to_string());

            // 要检查archive里面的文件和系统外面的路径是否都存在
            // 还要检查复制后是否会导致文件覆盖？
            if target_path.exists() {
                return Err(RestoreError::DuplicatedOrigin(
                    target_path.to_string_lossy().to_string(),
                ));
            }

            if !archive_path.exists() {
                return Err(RestoreError::ArchivedFileMissing(
                    archive_path.to_string_lossy().to_string(),
                ));
            }

            fs::rename(archive_path, target_path)?;
            list::mark_as_restored(id)?;
            return Ok(());
        }
        Err(e) => return Err(RestoreError::from(e)),
    }
}
