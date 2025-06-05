use crate::{as_fatal, must_ok};

use std::fs;

use crate::misc::{clap_mark, paths};
use crate::models::types::AutoIncrVars;
use crate::models::{error::ArchiverResult, serde_custom::SerdeJson};

/// 看一下自增主键的下一个值
/// - 不会更新
pub fn peek_next(name: &str) -> u32 {
    let auto_incr = must_ok!(load(), "Failed to parse auto increment file");
    match auto_incr {
        AutoIncrVars(m) => {
            if let Some(&id) = m.get(name) {
                return id + 1;
            }
            panic!(
                "{} Unknown auto increment variable: {}",
                clap_mark::error(),
                name
            );
        }
    }
}

/// 获取自增主键的下一个值，并更新自增变量
pub fn next(name: &str) -> u32 {
    let auto_incr = must_ok!(load(), "Failed to parse auto increment file");
    match auto_incr {
        AutoIncrVars(mut m) => {
            if let Some(&id) = m.get(name) {
                let next_id = id + 1;
                m.insert(name.to_string(), next_id);
                must_ok!(save(&AutoIncrVars(m)), "Cannot update AutoIncrVars");
                return next_id;
            }
            panic!(
                "{} Unknown auto increment variable: {}",
                clap_mark::error(),
                name
            );
        }
    }
}

fn save(auto_incr: &AutoIncrVars) -> ArchiverResult<()> {
    let json = as_fatal!(auto_incr.to_formatted_string())?;
    as_fatal!(fs::write(paths::AUTO_INCR_FILE_PATH.as_path(), json))?;
    Ok(())
}

fn load() -> ArchiverResult<AutoIncrVars> {
    let auto_incr_file = paths::AUTO_INCR_FILE_PATH.as_path();
    let content = as_fatal!(fs::read_to_string(auto_incr_file))?;
    let auto_incr = as_fatal!(AutoIncrVars::from_json_string(&content))?;
    Ok(auto_incr)
}
