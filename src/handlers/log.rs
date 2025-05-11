use crate::misc::{paths, write_entry};
use crate::models::errors::OperLogError;
use crate::models::types::{LogEntry, OperType};

use chrono::{Datelike, Local, NaiveDate};
use owo_colors::OwoColorize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn handler(interval: Option<String>) {
    match load(interval) {
        Ok(_) => {}
        Err(e) => println!("{}", e.to_string()),
    };
}

fn parse_date(date_str: Option<&str>, default_date: NaiveDate) -> Result<NaiveDate, OperLogError> {
    let d = match date_str {
        Some(s) => {
            if s == "*" {
                default_date
            } else {
                NaiveDate::parse_from_str(&format!("{}-01", s), "%Y%m-%d")?
            }
        }
        None => default_date,
    };
    Ok(d)
}

fn log_content(dates: &(NaiveDate, NaiveDate), content: &String, counter: &mut u32) {
    // 解析每行JSON
    for line in content.lines() {
        if line.trim().is_empty() {
            continue; // 跳过空行
        }
        match serde_json::from_str::<LogEntry>(line) {
            Ok(entry) => {
                let dt = NaiveDate::parse_from_str(&entry.time, "%Y-%m-%d %H:%M:%S");
                match dt {
                    Ok(dt) => {
                        if dt < dates.0 || dt > dates.1 {
                            continue; // 跳过不在范围内的日期
                        }

                        *counter += 1;
                        println!("{}", entry.to_str())
                    }
                    Err(_) => continue,
                }
            }
            Err(e) => {
                println!("{}: {}", "解析日志行失败".red(), e.to_string().yellow());
                continue;
            } // 跳过解析错误的行
        }
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
pub fn save(
    oper: OperType,
    arg: String,
    is_succ: bool,
    id: Option<u32>,
) -> Result<(), OperLogError> {
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
        oper,
        arg,
        id, // archive id，如果有的话
    };

    write_entry(&log_entry, log_file_path).map_err(|e| OperLogError::IoError(e.to_string()))?;
    println!("Operation log saved");
    Ok(())
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
fn load(interval: Option<String>) -> Result<(), OperLogError> {
    let log_dir = paths::logs_dir();

    // 下面开始规整入参的日期
    let default_start =
        NaiveDate::from_ymd_opt(1970, 1, 1).expect("Should not fail to create 1970-01-01");
    let default_end = Local::now().date_naive();
    let dates: (NaiveDate, NaiveDate) = match interval {
        Some(criteria) => {
            let mut iter = criteria.split_whitespace();
            let start = parse_date(iter.next(), default_start)?;
            let end = parse_date(iter.next(), default_end)?;

            if start > end {
                return Err(OperLogError::DateParseError(
                    "Start date cannot be greater than end date".to_string(),
                ));
            }

            (start, end)
        }
        None => (default_start, default_end),
    };

    let mut counter: u32 = 0;
    for year in dates.0.year()..=dates.1.year() {
        let log_file_path = log_dir.join(format!("{}.jsonl", year));
        if !log_file_path.exists() {
            continue;
        }

        let content = fs::read_to_string(log_file_path)?;
        log_content(&dates, &content, &mut counter);
    }

    if counter == 0 {
        println!("No logs found");
    }

    Ok(())
}
