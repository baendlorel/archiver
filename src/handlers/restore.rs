use owo_colors::OwoColorize;

use crate::{err_info, wrap_err, wrap_result};

use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use super::{list, log};
use crate::misc::{ForceToString, paths, status_mark};
use crate::models::{
    error::ArchiverError,
    types::{ListEntry, OperType},
};

pub fn handler(ids: &[u32]) {
    for id in ids {
        println!("Restoring id: {}", id.magenta());
        match restore(*id) {
            Ok(entry) => {
                println!(
                    "{} id:{} is successfully restored to '{}'",
                    status_mark::succ(),
                    id.magenta(),
                    entry.get_target_path()
                );
                log::succ(OperType::Restore, id.to_string(), Some(*id), None);
            }
            Err(e) => log::err(OperType::Restore, id.to_string(), Some(*id), e),
        }
    }
}

fn restore(id: u32) -> Result<ListEntry, ArchiverError> {
    let mut target_line_index: u32 = 0;
    let entry = wrap_result!(list::find(id, &mut target_line_index))?;
    if entry.is_restored {
        return Err(err_info!(format!(
            "id:{} has already been restored to '{}'.",
            id.magenta(),
            entry.get_target_path()
        )));
    }

    let target_name = OsString::from(&entry.target);
    let dir = PathBuf::from(OsString::from(&entry.dir));
    let target_path = dir.join(target_name);
    let archive_path = paths::ROOT_DIR.join(id.to_string());

    // 要检查archive里面的文件和系统外面的路径是否都存在
    // 还要检查复制后是否会导致文件覆盖
    if target_path.exists() {
        return Err(err_info!(format!(
            "Path '{}' already exists, please remove or rename it first",
            target_path.force_to_string()
        )));
    }

    if !archive_path.exists() {
        return Err(err_info!(format!(
            "The archive file id: {} does not exist",
            archive_path.force_to_string()
        )));
    }

    // 先确保上面两个不异常
    // 再确保原目录存在
    if !dir.exists() {
        wrap_err!(fs::create_dir_all(&dir))?;
    }

    wrap_err!(fs::rename(archive_path, target_path))?;
    wrap_result!(list::mark_as_restored(target_line_index))?;
    Ok(entry)
}
