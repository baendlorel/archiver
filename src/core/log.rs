use crate::{err_error, info, wrap_result};

use crate::cli::Operation;
use crate::misc::clap_mark;
use crate::misc::console::table::{Column, ColumnAlign, Table};
use crate::models::error::{ArchiverError, ArchiverResult};
use crate::models::types::LogLevel;
use crate::traits::{CustomColors, ResultExt};

mod parser;
mod sl;

// ! 写入失败时只会输出到控制台，不会重试写入

/// 写一条操作成功的日志
/// - 入参message仅供控制台展示
///     - 因为操作本身的成功已经蕴含了message的信息
pub fn succ(
    archive_id: impl Into<Option<u32>>,
    vault_id: impl Into<Option<u32>>,
    message: &String,
) {
    println!("{} {}", clap_mark::succ(), message);

    // message没必要写入，因为level和operation已携带成功信息
    sl::save(LogLevel::Success, archive_id, vault_id, None).allow_and_display();
}

/// 写入错误日志
/// - 会继承入参error对象的level和信息
pub fn error(e: ArchiverError) {
    e.display();
    let str = e.to_string();
    let level = e.level;
    sl::save(level, None, None, str).allow_and_display();
}

/// 输出一段字符串
pub fn fail(message: &str) {
    let e = err_error!("{}", message);
    let str = e.to_string();
    let level = e.level;
    sl::save(level, None, None, str).allow_and_display();
}

/// 保存系统自动生成的操作的日志
pub fn sys(
    oper: Operation,
    level: LogLevel,
    archive_id: impl Into<Option<u32>>,
    vault_id: impl Into<Option<u32>>,
    remark: String,
) {
    sl::save_sys(oper, level, archive_id, vault_id, remark).allow_and_display();
}

pub fn display(range: &Option<String>) -> ArchiverResult<()> {
    let (mut logs, reach_casual_limit) = wrap_result!(sl::load(range))?;
    logs.reverse();

    let cols = vec![
        Column::left("ID"),
        Column::left("Time"),
        Column::center("⚑"),
        Column::left("Operation"),
        Column::new("Remark", ColumnAlign::Left, (6, 25)),
    ];
    Table::display(cols, &logs);

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
