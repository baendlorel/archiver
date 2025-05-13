use crate::misc::{append_entry, paths};
use crate::models::errors::OperLogError;
use crate::models::types::{LogEntry, OperType};

use chrono::{Datelike, Local};
use owo_colors::OwoColorize;
use std::path::Path;
use std::{fs, u32};

pub fn handler(range: Option<String>) {
    if let Err(e) = load(range) {
        println!("{}", e.to_string())
    }
}

fn log_content(
    range: &(u32, u32),
    content: &String,
    counter: &mut u32,
) -> Result<(), OperLogError> {
    // 把年月取出来组成一个整数用于比较
    let parse_ym = |s: &String| -> Result<u32, OperLogError> {
        // 现在s基本认为一定是一个时间字符串
        let mut iter = s.split("-");
        let raw_year = iter.next().ok_or(OperLogError::DateParseError(format!(
            "Year parse failed for '{}'",
            s
        )))?;
        let raw_month = iter.next().ok_or(OperLogError::DateParseError(format!(
            "Month parse failed for '{}'",
            s
        )))?;

        let year = raw_year.parse::<u32>()?;
        let month = raw_month.parse::<u32>()?;

        if month > 12 || month < 1 {
            return Err(OperLogError::DateParseError(format!(
                "Month > 12, parse failed for '{}'",
                s
            )));
        }

        Ok(year * 100 + month)
    };

    // 解析每行JSON
    for line in content.lines() {
        if line.trim().is_empty() {
            continue; // 跳过空行
        }

        let result = serde_json::from_str::<LogEntry>(line);

        // 这里不需要报错反回去，只需要跳过报错部分，让程序运行更稳定些
        if let Ok(entry) = &result {
            let ym = parse_ym(&entry.time)?;
            if ym < range.0 || ym > range.1 {
                continue; // 跳过不在范围内的日期
            }

            *counter += 1;
            println!("{}", entry.to_log())
        }

        if let Err(e) = &result {
            println!("{}: {}", "Parse log failed".red(), e.to_string().yellow());
            continue;
        }
    }

    Ok(())
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
    remark: Option<String>,
) -> Result<(), OperLogError> {
    // 获取日志文件路径
    let log_file_path =
        paths::LOGS_DIR.join(Path::new(format!("{}.jsonl", Local::now().year()).as_str()));

    // 确保日志目录存在
    // 获取当前时间
    let opered_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let normalized_remark = if oper == OperType::Archive {
        let full_path = paths::CWD.join(arg.clone());
        full_path.to_string_lossy().to_string()
    } else {
        remark.unwrap_or("".to_string())
    };

    // 准备日志内容
    let log_entry = LogEntry {
        time: opered_at,
        is_succ,
        oper,
        arg,
        remark: normalized_remark,
        id,
    };

    append_entry(&log_entry, log_file_path).map_err(|e| OperLogError::IoError(e.to_string()))?;
    Ok(())
}

/// Loads and filters operation logs
///
/// Loads operation logs from the current year's log file, can filter based on provided criteria.
///
/// # Parameters
///
/// * `criteria` - Optional filtering criteria. Can pass time range like `YYYYMM-YYYYMM` | `YYYYMM` | `*-YYYYMM`.
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
fn load(range: Option<String>) -> Result<(), OperLogError> {
    // 考虑到日期本质上是一个不定型进制数，可以考虑直接转为数字来对比大小
    let range = parse_range(range)?;
    let year_range = (range.0 / 100, range.1 / 100);

    let years = paths::get_all_logs_year();
    let mut counter: u32 = 0;
    for year in years {
        // 跳过不在范围内的年份
        if year < year_range.0 || year > year_range.1 {
            continue;
        }

        let log_file_path = paths::get_log_path(year);
        if !log_file_path.exists() {
            continue;
        }

        let content = fs::read_to_string(log_file_path)?;
        log_content(&range, &content, &mut counter)?;
    }

    if counter == 0 {
        println!("No logs found");
    }

    Ok(())
}

fn parse_range(range: Option<String>) -> Result<(u32, u32), OperLogError> {
    let default_a = u32::MIN;
    let default_b = u32::MAX;

    if range.is_none() {
        return Ok((default_a, default_b));
    }

    let range = &range.unwrap();

    let is_valid_ym = |s: &String| -> Result<bool, OperLogError> {
        let is_numeric = s.chars().all(|c| c.is_numeric());
        let is_valid_len = s.len() > 2;
        if !is_numeric || !is_valid_len {
            return Ok(false);
        }

        let raw_month = s[(s.len() - 2)..s.len()].parse::<u32>()?;
        let valid_month = raw_month <= 12 && raw_month >= 1;

        Ok(valid_month)
    };

    let parse = |s: &String, default_value: u32| -> Result<u32, OperLogError> {
        if s == "*" {
            return Ok(default_value);
        }

        if is_valid_ym(s)? {
            return Ok(s.parse::<u32>()?);
        } else {
            return Err(OperLogError::DateParseError(
                "Date must be like `YYYYMM`".to_string(),
            ));
        }
    };

    // TODO 入参为202039，月份越界，报错需要精确化，现为DateParseError: Must give args like 202501, 202501-202506,*-202501。
    if is_valid_ym(range)? {
        return Ok((range.parse::<u32>()?, default_b));
    }

    if let Some((a_str, b_str)) = range.split_once('-') {
        let a = parse(&a_str.to_string(), default_a)?;
        let b = parse(&b_str.to_string(), default_b)?;

        if a > b {
            return Err(OperLogError::DateParseError(
                "Start date > end date".to_string(),
            ));
        }
        return Ok((a, b));
    }

    Err(OperLogError::DateParseError(
        "Must give args like 202501, 202501-202506,*-202501".to_string(),
    ))
}
