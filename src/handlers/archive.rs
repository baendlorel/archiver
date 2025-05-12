use owo_colors::OwoColorize;
use std::fs;

use super::{list, log};
use crate::{
    misc::{force_no_loss, paths},
    models::{errors::ArchiveError, types::OperType},
};

pub fn handler(target: String) {
    println!("Archiving {}", target.green());
    match archive(&target) {
        Ok(id) => {
            let _ = log::save(OperType::Archive, target.clone(), true, Some(id), None);
            println!("'{}' is successfully archived", target)
        }
        Err(e) => {
            let _ = log::save(
                OperType::Archive,
                target.clone(),
                false,
                None,
                Some(e.to_string()),
            );
            println!("{}", e.to_string());
        }
    };
}

fn archive(target: &String) -> Result<u32, ArchiveError> {
    // 不能trim不能检测为空，否则无法正确处理带空格的文件/文件夹名
    let cwd = paths::CWD.clone();
    let target_path = cwd.join(target);

    // 目标不存在则报错
    if !target_path.exists() {
        return Err(ArchiveError::TargetNotFound(
            target_path.to_string_lossy().to_string(),
        ));
    }

    // 必须无损转换OsString
    let cwd_str = match force_no_loss(cwd.as_os_str()) {
        Ok(str) => str,
        Err(_) => {
            return Err(ArchiveError::InvalidCwd(
                "Failed to convert current directory to string".to_string(),
            ));
        }
    };

    let target_name = target_path.file_name().ok_or(ArchiveError::InvalidTarget(
        "Failed to get target name".to_string(),
    ))?;

    // 必须无损转换OsString
    let target_name_str = match force_no_loss(target_name) {
        Ok(str) => str,
        Err(_) => {
            return Err(ArchiveError::InvalidTarget(
                "Failed to convert target name to string. Please use utf8 chars to name the target"
                    .to_string(),
            ));
        }
    };

    // 都没有异常，那么开始归档
    let is_dir = target_path.is_dir(); // 不能在rename之后调用，否则目录已经没了，百分百不是
    let root = paths::ROOT_DIR.clone();
    let next_id = paths::auto_incr_id();

    fs::rename(&target_path, root.join(next_id.to_string()))?;

    list::insert(next_id, target_name_str, is_dir, cwd_str)?;

    Ok(next_id)
}
