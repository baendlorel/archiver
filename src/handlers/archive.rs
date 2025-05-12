use std::{f32::consts::E, fs};

use owo_colors::OwoColorize;

use super::{list, log};
use crate::{
    misc::paths,
    models::{errors::ArchiveError, types::OperType},
};

pub fn handler(target: String) {
    println!("归档目录 {}", target.green());
    match archive(&target) {
        Ok(_) => println!("{} is successfully archived", target),
        Err(e) => {
            let _ = log::save(OperType::Archive, target.clone(), false, None);
            println!("{}", e.to_string());
        }
    };
}

fn archive(target: &String) -> Result<(), ArchiveError> {
    let target = &target.trim().to_string();
    let cwd = paths::cwd();
    let target_path = cwd.join(target);

    if target.is_empty() {
        return Err(ArchiveError::InvalidTarget(
            "Target path cannot be empty".to_string(),
        ));
    }

    if !target_path.exists() {
        return Err(ArchiveError::TargetNotFound(
            target_path.to_string_lossy().to_string(),
        ));
    }

    // 目标存在，那么开始归档
    let root = paths::root_dir();
    let next_id = paths::auto_incr_id();

    fs::rename(&target_path, root.join(next_id.to_string()))?;

    let cwd_str = match cwd.to_str() {
        Some(str) => str.to_string(),
        None => {
            return Err(ArchiveError::InvalidCwd(
                "Failed to convert current directory to string".to_string(),
            ));
        }
    };

    list::save(next_id, target.clone(), target_path.is_dir(), cwd_str)?;
    log::save(OperType::Archive, target.clone(), true, Some(next_id))?;

    Ok(())
}
