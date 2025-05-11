use chrono::{Datelike, Local, NaiveDate};
use owo_colors::OwoColorize;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::vec;

use crate::misc::paths;
use crate::models::errors::OperLogError;
use crate::models::types::{LogEntry, OperType};

pub fn handler(time_interval: Option<String>) {
    println!("归档日志");
    if time_interval.is_some() {
        println!("时间区间：{}", time_interval.unwrap().green());
    } else {
        println!("时间区间：所有");
    }
}

/// Saves an operation log
///
/// Records operations in a year-named JSON Lines format log file.
///
/// # Parameters
///
/// * `oper` - Operation type
/// * `arg` - Operation parameter, such as file path
/// * `is_succ` - Whether the operation was successful
/// * `id` - Archive ID (if oper is `Archive` or `Restore`)
///
/// # Returns
///
/// Returns `Ok(())` on success, or an `Err` containing error information on failure
///
/// # Errors
///
/// May return errors in the following cases:
/// * Unable to get user home directory
/// * Unable to create log directory
/// * JSON serialization failure
/// * File writing failure
pub fn save(oper: OperType, arg: String, is_succ: bool, id: u32) -> Result<(), OperLogError> {
    // 获取日志文件路径
    let log_dir = paths::logs_dir();
    let log_file_path = log_dir.join(Path::new(format!("{}.jsonl", Local::now().year()).as_str()));

    // 确保日志目录存在
    // 获取当前时间
    let opered_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 准备日志内容
    let log_entry = LogEntry {
        time: opered_at,
        status: if is_succ { "succ" } else { "fail" }.to_string(),
        oper: oper.to_str().to_string(),
        arg,
        id, // archive id，如果有的话
    };

    // 序列化为JSON
    let json_line = serde_json::to_string(&log_entry).map_err(|e| OperLogError::from(e))?;

    // 以追加模式打开文件
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)?;

    // 写入日志
    file.write_all(json_line.as_bytes())?;
    file.write_all(b"\n")?;

    println!("操作日志已保存");
    return Ok(());
}

/// Loads and filters operation logs
///
/// Loads operation logs from the current year's log file, can filter based on provided criteria.
///
/// # Parameters
///
/// * `criteria` - Optional filtering criteria. Can pass time interval like `YYYYMM YYYYMM` | `YYYYMM` or `YYYYMM *` | `* YYYYMM`. Will use current year as default
///
/// # Returns
///
/// Returns `Ok(())` on success, or an `Err` containing error information on failure
///
/// # Errors
///
/// May return errors in the following cases:
/// * Unable to get user home directory
/// * Unable to read log file
///
/// # Notes
///
/// If the log file doesn't exist, it will return success without loading any records.
/// Failed log line parsing will be skipped and warning messages will be output.
fn load(criteria: Option<String>) -> Result<(), OperLogError> {
    // 获取日志文件路径
    let log_dir = paths::logs_dir();

    // 下面开始规整入参的日期
    let interval = match criteria {
        Some(criteria) => {
            let iter = criteria.split_whitespace();
            let start = match iter.next() {
                Some(s) => NaiveDate::parse_from_str(&format!("{}-01", s), "%Y%m-%d")?,
                None => NaiveDate::from_ymd_opt(1970, 1, 1).expect("日期解析失败"),
            };
            let end = match iter.next() {
                Some(s) => NaiveDate::parse_from_str(&format!("{}-01", s), "%Y%m-%d")?,
                None => NaiveDate::now(),
            };

            if start > end {
                return Err(OperLogError::DateParseError(
                    "Start date cannot be greater than end date".to_string(),
                ));
            }

            (start, end)
        }
        None => (NaiveDate::from_ymd_opt(1970, 1, 1)?, NaiveDate::now()),
    };

    for year in interval.0.year()..=interval.1.year() {
        let log_file_path = log_dir.join(format!("{}.jsonl", year));

        // 读取文件内容
        let content = fs::read_to_string(log_file_path)?;

        // 解析每行JSON
        let mut entries = Vec::new();
        for line in content.lines() {
            if !line.trim().is_empty() {
                match serde_json::from_str::<LogEntry>(line) {
                    Ok(entry) => entries.push(entry),
                    Err(e) => eprintln!("解析日志行失败: {}", e),
                }
            }
        }
    }
    println!("加载了 {} 条操作日志", entries.len());
    Ok(())
}
