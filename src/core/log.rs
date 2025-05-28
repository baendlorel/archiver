use crate::{allow, wrap_result};

use crate::misc::mark;
use crate::models::error::{ArchiverError, ArchiverResult};
use crate::models::types::LogLevel;
use crate::models::types::Operation;

mod parser;
mod sl;

/// ! 写入失败时只会输出到控制台，不会重试写入

/// 写入成功的日志
/// - 会继承入参msg对象的信息
pub fn succ(archive_id: Option<u32>, vault_id: Option<u32>, message: &String) {
    println!("{} {}", mark::succ(), message);

    // message没必要写入，因为level和operation已携带成功信息
    allow!(sl::save(LogLevel::Success, archive_id, vault_id, None));
}

/// 写入错误日志
/// - 会继承入参error对象的level和信息
pub fn fail(e: ArchiverError) {
    e.display();
    let str = e.to_string();
    let level = e.level;
    allow!(sl::save(level, None, None, Some(str)));
}

/// 保存系统自动生成的操作的日志
pub fn sys(
    oper: Operation,
    level: LogLevel,
    archive_id: Option<u32>,
    vault_id: Option<u32>,
    remark: String,
) {
    allow!(sl::save_system_oper(
        oper, level, archive_id, vault_id, remark
    ));
}

pub fn display(range: &Option<String>) -> ArchiverResult<()> {
    let (logs, reach_casual_limit) = wrap_result!(sl::load(range))?;
    logs.iter()
        .rev()
        .for_each(|l| println!("{}", l.to_display()));

    if reach_casual_limit {
        println!("Recent {} logs displayed.", logs.len());
    }

    Ok(())
}
