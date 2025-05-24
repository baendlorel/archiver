use owo_colors::OwoColorize;

use crate::{err_info, wrap_err_fatal, wrap_result};

use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use super::{list, log};
use crate::misc::{ForceToString, paths};
use crate::models::{
    error::ArchiverError,
    types::{ListEntry, OperType},
};

pub fn handler(ids: &[u32]) {
    let oper = OperType::Restore;
    for id in ids {
        println!("Restoring id:{}", id.magenta());
        match restore(*id) {
            Ok(entry) => {
                let msg = format!(
                    "id:{} is successfully restored to '{}'",
                    id.magenta(),
                    entry.get_target_path()
                );
                log::succ(&oper, &id.to_string(), Some(*id), &msg);
            }
            Err(e) => log::err(&oper, &id.to_string(), Some(*id), e),
        }
    }
}

fn restore(id: u32) -> Result<ListEntry, ArchiverError> {
    let (entry, line_index, list_path) = wrap_result!(list::find(id))?;
    if entry.is_restored {
        return err_info!(
            "id:{} has already been restored to '{}'",
            id,
            entry.get_target_path()
        );
    }

    let target_name = OsString::from(&entry.target);
    let dir = PathBuf::from(OsString::from(&entry.dir));
    let target_path = dir.join(target_name);
    let archive_path = paths::get_archived_path(id, entry.vault_id);

    // 要检查archive里面的文件和系统外面的路径是否都存在
    // 还要检查复制后是否会导致文件覆盖
    if target_path.exists() {
        return err_info!(
            "Path '{}' already exists, please remove or rename it first",
            target_path.force_to_string()
        );
    }

    if !archive_path.exists() {
        return err_info!(
            "The archive file id:{} does not exist",
            archive_path.force_to_string()
        );
    }

    // 先确保上面两个不异常
    // 再确保原目录存在
    if !dir.exists() {
        wrap_err_fatal!(fs::create_dir_all(&dir))?;
    }

    wrap_err_fatal!(fs::rename(archive_path, target_path))?;
    wrap_result!(mark_as_restored(line_index, &list_path))?;
    Ok(entry)
}

/// 只会在目标已经被restored之后调用
fn mark_as_restored(line_index: usize, list_path: &std::path::Path) -> Result<(), ArchiverError> {
    // 读取整个文件
    let content = wrap_err_fatal!(fs::read_to_string(&list_path))?;

    let mut lines: Vec<&str> = content.lines().collect();
    let target_line = lines[line_index];
    let modified_line = {
        // 把这条记录标记为restored
        let mut entry = wrap_err_fatal!(serde_json::from_str::<ListEntry>(target_line))?;
        entry.is_restored = true;
        wrap_err_fatal!(serde_json::to_string(&entry))?
    };

    lines[line_index] = modified_line.as_str();

    // 将内容写回文件
    wrap_err_fatal!(fs::write(&list_path, lines.join("\n") + "\n"))?;

    Ok(())
}
