use crate::{err_info, err_warn, misc::auto_incr, uoe_option, wrap_err_fatal, wrap_result};

use std::{ffi::OsStr, fs, path::PathBuf};

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
    let cwd: std::path::PathBuf = paths::CWD.clone();
    let target_path = wrap_err_fatal!(cwd.join(target).canonicalize())?;
    let vault_path = paths::get_default_vault_path();

    // 目标不存在则报错
    if !target_path.exists() {
        return err_info!("'{}' does not exist.", target_path.force_to_string());
    }

    if paths::ROOT_DIR.starts_with(&target_path) {
        return err_warn!(
            "Target cannot be a parent directory of archiver or itself. Got '{}'",
            target
        );
    }

    // & 路径相关性检测：不能归档归档器本身、其父目录、其子目录
    if invalid_target(&target_path) {
        return err_warn!(
            "Target cannot be the archiver directory, its parent, or its inner object. Got '{}'",
            target
        );
    }

    // 必须无损转换OsString
    let target_dir =
        uoe_option!(target_path.parent(), "Fail to get target directory").force_to_string();

    let target_name: &OsStr = uoe_option!(target_path.file_name(), "Fail to get target name");

    // 必须无损转换OsString
    let target_name_str = force_no_loss_string(target_name);

    // 都没有异常，那么开始归档
    let is_dir = target_path.is_dir(); // 不能在rename之后调用，否则目录已经没了，百分百不是
    let next_id = auto_incr::archive_id::next();
    let archived_path = vault_path.join(next_id.to_string());

    wrap_err_fatal!(fs::rename(&target_path, archived_path))?;

    wrap_result!(list::insert(next_id, target_name_str, is_dir, target_dir))?;

    auto_incr::archive_id::update(next_id);

    Ok(next_id)
}

/// 属于Archiver自己的文件夹，以及其父文件夹，不允许put
fn invalid_target(target_path: &PathBuf) -> bool {
    let root = paths::ROOT_DIR.as_path();

    // 这句可以判定target是不是Archiver及其子目录
    if target_path.starts_with(root) {
        return true;
    }

    // 这句可以判定target是不是Archiver的父目录
    for ancestor in root.ancestors() {
        if target_path == ancestor {
            return true;
        }
    }

    false
}
