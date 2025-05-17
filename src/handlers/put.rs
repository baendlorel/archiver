use crate::{err, misc::status_mark, wrap_err, wrap_result};

use owo_colors::OwoColorize;
use std::fs;

use super::{list, log};
use crate::{
    misc::{ForceToString, force_no_loss_string, paths},
    models::{error::ArchiverError, types::OperType},
};

pub fn handler(targets: Vec<String>) {
    for target in targets {
        println!("Putting '{}' into archive", target);
        match archive(&target) {
            Ok(id) => {
                println!(
                    "{} '{}' is successfully archived, id: {}",
                    status_mark::succ(),
                    target,
                    id.magenta()
                );
                log::succ(OperType::Put, target.clone(), Some(id), None);
            }
            Err(e) => log::err(OperType::Put, target.clone(), None, e),
        };
    }
    println!("Use `arv list` to check the archived list");
}

fn archive(target: &String) -> Result<u32, ArchiverError> {
    // 不能trim不能检测为空，否则无法正确处理带空格的文件/文件夹名
    let cwd = paths::CWD.clone();
    let target_path = cwd.join(target);

    // 目标不存在则报错
    if !target_path.exists() {
        return Err(err!(format!(
            "Target '{}' does not exist in current directory.",
            target
        )));
    }

    // 必须无损转换OsString
    let cwd_str = cwd.force_to_string();

    let target_name: &std::ffi::OsStr = target_path
        .file_name()
        .ok_or(err!("Fail to get target name".to_string()))?;

    // 必须无损转换OsString
    let target_name_str = force_no_loss_string(target_name);

    // 都没有异常，那么开始归档
    let is_dir = target_path.is_dir(); // 不能在rename之后调用，否则目录已经没了，百分百不是
    let root = paths::ROOT_DIR.clone();
    let next_id = paths::auto_incr_id();

    wrap_err!(fs::rename(&target_path, root.join(next_id.to_string())))?;

    wrap_result!(list::insert(next_id, target_name_str, is_dir, cwd_str))?;

    Ok(next_id)
}
