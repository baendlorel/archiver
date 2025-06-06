use crate::{err_error, err_warn, info, wrap_result};

use crate::cli::{OperSource, Operation};
use crate::misc::clap_mark;
use crate::misc::console::table::{Column, ColumnAlign, Table};
use crate::models::error::{ArchiverError, ArchiverResult};
use crate::models::types::LogLevel;
use crate::traits::{CustomColors, ResultExt};

mod parser;
mod sl;

// ! 写入失败时只会输出到控制台，不会重试写入

/// 写一条操作成功的日志
/// - 取消了入参message，因为不需要记入remark的字段不应该耦合于此
pub fn succ(archive_ids: impl Into<Option<Vec<u32>>>, vault_ids: impl Into<Option<Vec<u32>>>) {
    // message没必要写入，因为level和operation已携带成功信息
    sl::save_simple(LogLevel::Success, archive_ids, vault_ids, None).allow_and_display();
}

/// 写入错误日志
/// - 会继承入参error对象的level和信息
pub fn error(e: ArchiverError) {
    e.display();
    let str = e.to_string();
    let level = e.level;
    sl::save_simple(level, None, None, str).allow_and_display();
}

/// 输出一段字符串，并以error级别记录日志
/// - error日志需要把message记入remark字段
pub fn fail(message: &str) {
    let e = err_error!("{}", message);
    let str = e.to_string();
    let level = e.level;
    sl::save_simple(level, None, None, str).allow_and_display();
}

/// 保存系统自动生成的操作的日志
pub fn sys(
    oper: Operation,
    level: LogLevel,
    archive_ids: impl Into<Option<Vec<u32>>>,
    vault_ids: impl Into<Option<Vec<u32>>>,
) {
    if !matches!(oper.source, OperSource::System) {
        let e = err_warn!("User operations should not call this function directly");
        e.display();
        return;
    }
    sl::save(oper, level, archive_ids, vault_ids, String::new()).allow_and_display();
}

pub fn display(range: &Option<String>) -> ArchiverResult<()> {
    let (mut logs, reach_casual_limit) = wrap_result!(sl::load(range))?;
    logs.reverse();

    Table::display(&logs);

    if reach_casual_limit {
        println!("Recent {} logs displayed.", logs.len());
    }

    Ok(())
}

pub fn display_by_id(id: u32) -> ArchiverResult<()> {
    // 这里没办法只能加载全部logs
    let logs = wrap_result!(sl::find(|entry| entry.id == id))?;
    if logs.len() == 0 {
        return info!("Log with id: {} not found.", id.styled_id());
    }
    if logs.len() > 1 {
        println!(
            "{} Multiple logs found with id: {}. Will display all of them.",
            clap_mark::warn(),
            id
        );
    }

    for log in logs {
        log.display();
    }

    Ok(())
}
