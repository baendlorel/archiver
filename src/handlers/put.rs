use crate::{err_fatal_from_str, err_info, err_warn, wrap_err_fatal, wrap_result};

use std::fs;

use super::{list, log};
use crate::{
    misc::{ForceToString, force_no_loss_string, paths},
    models::{error::ArchiverError, types::OperType},
};

pub fn handler(targets: &[String]) {
    let oper = OperType::Put;
    for target in targets {
        println!("Putting '{}' into archive", target);
        match archive(&target) {
            Ok(id) => {
                let msg = format!("'{}' is successfully archived, id: {}", target, id);
                log::succ(&oper, target, Some(id), &msg);
            }
            Err(e) => log::err(&oper, target, None, e),
        };
    }
    println!("Use `arv list` to check the archived list");
}

fn archive(target: &str) -> Result<u32, ArchiverError> {
    // 不能trim不能检测为空，否则无法正确处理带空格的文件/文件夹名
    let cwd = paths::CWD.clone();
    let target_path = cwd.join(target);

    // 目标不存在则报错
    if !target_path.exists() {
        return err_info!("'{}' does not exist in current directory.", target);
    }

    if paths::ROOT_DIR.starts_with(&target_path) {
        return err_warn!(
            "'{}' cannot be a parent of archiver directory or itself.",
            target,
        );
    }

    // 必须无损转换OsString
    let cwd_str = cwd.force_to_string();

    let target_name: &std::ffi::OsStr = target_path
        .file_name()
        .ok_or(err_fatal_from_str!("Fail to get target name"))?; // 需要fatal

    // 必须无损转换OsString
    let target_name_str = force_no_loss_string(target_name);

    // 都没有异常，那么开始归档
    let is_dir = target_path.is_dir(); // 不能在rename之后调用，否则目录已经没了，百分百不是
    let root = paths::ROOT_DIR.clone();
    let next_id = paths::auto_incr_id();

    wrap_err_fatal!(fs::rename(&target_path, root.join(next_id.to_string())))?;

    wrap_result!(list::insert(next_id, target_name_str, is_dir, cwd_str))?;

    Ok(next_id)
}
