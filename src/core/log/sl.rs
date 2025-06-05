use crate::wrap_result;

use chrono::Datelike;
use std::u32;

use super::parser;
use crate::cli::{FULL_CMD, Operation};
use crate::misc::{dt, jsonl, mark, paths};
use crate::models::error::ArchiverResult;
use crate::models::types::{LogEntry, LogLevel};
use crate::traits::ForceToString;

/// 在不加range直接arv log的时候，只输出最近这么多条
/// 避免日志太多
const CASUAL_LIMIT: usize = 16;

/// 保存日志
pub fn save(
    oper: Operation,
    level: LogLevel,
    archive_id: impl Into<Option<u32>>,
    vault_id: impl Into<Option<u32>>,
    message: String,
) -> ArchiverResult<()> {
    // 准备日志内容
    let log_entry = LogEntry::new(oper, level, message, archive_id.into(), vault_id.into());
    // 获取日志文件路径
    let log_file_path = paths::get_log_path(dt::now_year());
    wrap_result!(jsonl::append(&log_entry, &log_file_path))?;
    Ok(())
}

/// 省略oper的保存日志
pub fn save_simple(
    level: LogLevel,
    archive_id: impl Into<Option<u32>>,
    vault_id: impl Into<Option<u32>>,
    message: impl Into<Option<String>>,
) -> ArchiverResult<()> {
    let oper = FULL_CMD.to_operation();
    let message = message.into().unwrap_or(String::new());
    wrap_result!(save(oper, level, archive_id, vault_id, message))?;
    Ok(())
}

pub fn find(condition: impl Fn(&LogEntry) -> bool) -> ArchiverResult<Vec<LogEntry>> {
    let years = paths::get_years_desc();
    let mut logs: Vec<LogEntry> = vec![];
    for year in years {
        let log_file_path = paths::get_log_path(year);
        // 这里不应该没有，严谨起见做判定输出
        if !log_file_path.exists() {
            println!(
                "{} path '{}' not found",
                mark::warn(),
                log_file_path.force_to_string()
            );
            continue;
        }
        let cur_logs = jsonl::load::<LogEntry>(&log_file_path)?;
        for log in cur_logs {
            if condition(&log) {
                logs.push(log);
            }
        }
    }

    Ok(logs)
}

/// 加载日志
///
/// 返回值为元组：（日志数组，是否到达随便看看限制）
pub fn load(range: &Option<String>) -> ArchiverResult<(Vec<LogEntry>, bool)> {
    // 是否随便看看，如果没有给定range，那么别输出过多条数
    let casual = range.is_none();

    let (a, b) = wrap_result!(parser::normalize_range(range))?;
    let (ya, yb) = (a.year(), b.year());

    let years = paths::get_years_desc();
    let mut logs: Vec<LogEntry> = vec![];
    'year_loop: for year in years {
        // 跳过不在范围内的年份
        if year < ya || year > yb {
            continue;
        }

        let log_file_path = paths::get_log_path(year);
        // 这里不应该没有，严谨起见做判定输出
        if !log_file_path.exists() {
            println!(
                "{} path '{}' not found",
                mark::warn(),
                log_file_path.force_to_string()
            );
            continue;
        }

        let cur_logs = jsonl::load::<LogEntry>(&log_file_path)?;

        for l in cur_logs.into_iter().rev() {
            if l.opered_at < a || l.opered_at > b {
                continue; // 跳过不在范围内的日期
            }
            logs.push(l);
            // 如果没设置范围，只是随便看看日志，那么不要打得太多
            // 同时在load和log_content使用方可生效
            if casual && logs.len() >= CASUAL_LIMIT {
                break 'year_loop;
            }
        }
    }

    // 如果是随便看看而且到达最大值，那么提示可以看更多
    let reach_casual_limit = casual && logs.len() == CASUAL_LIMIT;

    if logs.len() == 0 {
        println!("No logs found");
    }

    Ok((logs, reach_casual_limit))
}
