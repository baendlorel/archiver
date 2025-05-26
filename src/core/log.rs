use crate::models::types::Operation;
use crate::{log_if_err, wrap_result};

use crate::misc::mark;
use crate::models::error::{ArchiverError, ArchiverResult};

mod parser;

mod sl;

pub fn succ(archive_id: Option<u32>, vault_id: Option<u32>, msg: &String) {
    println!("{} {}", mark::succ(), msg);
    log_if_err!(sl::save(true, archive_id, vault_id, None));
}

pub fn fail(e: ArchiverError) {
    e.display();
    log_if_err!(sl::save(false, None, None, Some(e.to_string())));
}

/// 保存系统自动生成的操作的日志
pub fn save_system_oper(
    oper: &Operation,
    is_succ: bool,
    archive_id: Option<u32>,
    vault_id: Option<u32>,
    remark: String,
) {
    log_if_err!(sl::save_system_oper(
        oper, is_succ, archive_id, vault_id, remark
    ));
}

pub fn display(range: &Option<String>) -> ArchiverResult<()> {
    let (logs, reach_casual_limit, casual_limit) = wrap_result!(sl::load(range))?;
    logs.iter()
        .rev()
        .for_each(|l| println!("{}", l.to_display()));
    if reach_casual_limit {
        println!("Recent {} logs displayed.", casual_limit);
    }
    Ok(())
}
