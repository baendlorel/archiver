use crate::{warn, wrap_result};

use chrono::Datelike;
use std::u32;

use super::parser;
use crate::cli::{FULL_CMD, short};
use crate::misc::{ForceToString, dt, jsonl, mark, paths};
use crate::models::error::ArchiverResult;
use crate::models::types::{LogEntry, LogLevel, OperSource, Operation};

/// 在不加range直接arv log的时候，只输出最近这么多条
/// 避免日志太多
const CASUAL_LIMIT: usize = 15;

pub fn save_system_oper(
    oper: &Operation,
    level: LogLevel,
    archive_id: Option<u32>,
    vault_id: Option<u32>,
    remark: String,
) -> ArchiverResult<()> {
    // 保证是系统生成的操作才能调用
    match oper.source {
        // 系统操作
        OperSource::System => {}
        // 用户操作
        _ => return warn!("User operations should not call this function directly"),
    }

    // 准备日志内容
    let log_entry = LogEntry::new(oper, level, remark, archive_id, vault_id);

    // 获取日志文件路径
    let log_file_path = paths::get_log_path(dt::now_year());
    wrap_result!(jsonl::append(&log_entry, &log_file_path))?;
    Ok(())
}

pub fn save(
    level: LogLevel,
    archive_id: Option<u32>,
    vault_id: Option<u32>,
    remark: Option<String>,
) -> ArchiverResult<()> {
    let oper = FULL_CMD.to_operation();
    let remark = if oper.main == short::main::PUT && level.is_succ() {
        let full_paths: Vec<String> = oper
            .args
            .iter()
            .map(|a| match paths::CWD.join(a).canonicalize() {
                Ok(p) => p.force_to_string(),
                Err(e) => {
                    println!(
                        "{} Failed to canonicalize path '{}': {}",
                        mark::warn(),
                        a,
                        e
                    );
                    a.clone() // 如果失败，保留原路径
                }
            })
            .collect();
        full_paths.join(" ")
    } else {
        remark.unwrap_or(String::new())
    };

    // 准备日志内容
    let log_entry = LogEntry::new(&oper, level, remark, archive_id, vault_id);

    // 获取日志文件路径
    let log_file_path = paths::get_log_path(dt::now_year());
    wrap_result!(jsonl::append(&log_entry, &log_file_path))?;
    Ok(())
}

/// 加载日志
///
/// 返回值为三元组：（日志数组，是否到达随便看看限制，随便看看限制值）
pub fn load(range: &Option<String>) -> ArchiverResult<(Vec<LogEntry>, bool)> {
    // 是否随便看看，如果没有给定range，那么别输出过多条数
    let casual = range.is_none();

    // 考虑到日期本质上是一个不定型进制数，可以考虑直接转为数字来对比大小
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
