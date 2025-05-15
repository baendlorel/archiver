use chrono::{Datelike, Local};
use owo_colors::OwoColorize;
use std::{fs, u32};

use crate::misc::{ForceToString, append_entry, paths};
use crate::models::error::ArchiverError;
use crate::models::types::{LogEntry, OperType};
use crate::{err, wrap_err, wrap_result};

pub fn handler(range: Option<String>) {
    if let Err(e) = load(range) {
        println!("{}", e.to_string())
    }
}

fn log_content(
    range: &(u32, u32),
    content: &String,
    counter: &mut u32,
) -> Result<(), ArchiverError> {
    // 把年月取出来组成一个整数用于比较
    let parse_ym = |s: &String| -> Result<u32, ArchiverError> {
        // 现在s基本认为一定是一个时间字符串
        let mut iter = s.split("-");
        let raw_year = iter
            .next()
            .ok_or(err!(format!("Year parse failed for '{}'", s)))?;
        let raw_month = iter
            .next()
            .ok_or(err!(format!("Month parse failed for '{}'", s)))?;

        let year = wrap_err!(raw_year.parse::<u32>())?;
        let month = wrap_err!(raw_month.parse::<u32>())?;

        if month > 12 || month < 1 {
            return Err(err!(format!("Month > 12, parse failed for '{}'", s)));
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

pub fn write(oper: OperType, arg: String, is_succ: bool, id: Option<u32>, remark: Option<String>) {
    if let Err(e) = save(oper, arg, is_succ, id, remark) {
        panic!("{}", e.to_string())
    }
}

fn save(
    oper: OperType,
    arg: String,
    is_succ: bool,
    id: Option<u32>,
    remark: Option<String>,
) -> Result<(), ArchiverError> {
    // 获取日志文件路径
    let log_file_path = paths::get_log_path(Local::now().year() as u32);

    // 确保日志目录存在
    // 获取当前时间
    let opered_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let normalized_remark = if oper == OperType::Put {
        let full_path = paths::CWD.join(arg.clone());
        full_path.force_to_string()
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

    wrap_result!(append_entry(&log_entry, log_file_path))?;
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
fn load(range: Option<String>) -> Result<(), ArchiverError> {
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

        let content = wrap_err!(fs::read_to_string(log_file_path))?;
        wrap_result!(log_content(&range, &content, &mut counter))?;
    }

    if counter == 0 {
        println!("No logs found");
    }

    Ok(())
}

fn parse_range(range: Option<String>) -> Result<(u32, u32), ArchiverError> {
    let default_a = u32::MIN;
    let default_b = u32::MAX;

    if range.is_none() {
        return Ok((default_a, default_b));
    }

    let range = &range.unwrap();

    let is_parsable = |s: &String| -> bool { s == "*" || s.chars().all(|c| c.is_numeric()) };

    let parse = |s: &String, default_value: u32| -> Result<u32, ArchiverError> {
        if s == "*" {
            return Ok(default_value);
        }

        let is_valid_len = s.len() > 2;
        if !is_valid_len {
            return Err(err!("Length of date string must > 2"));
        }

        let raw_month = wrap_err!(s[(s.len() - 2)..s.len()].parse::<u32>())?;
        if raw_month > 12 || raw_month < 1 {
            return Err(err!(format!("Month must be 1~12. Got '{}'", raw_month)));
        }

        Ok(wrap_err!(s.parse::<u32>())?)
    };

    if is_parsable(range) {
        return Ok((parse(range, default_a)?, default_b));
    }

    if let Some((a_str, b_str)) = range.split_once('-') {
        if !is_parsable(&a_str.to_string()) {
            return Err(err!(format!(
                "Start date is not * or contains letters other than digits. Got '{}'",
                a_str
            )));
        }
        if !is_parsable(&b_str.to_string()) {
            return Err(err!(format!(
                "End date is not * or contains letters other than digits. Got '{}'",
                b_str
            )));
        }

        let a = parse(&a_str.to_string(), default_a)?;
        let b = parse(&b_str.to_string(), default_b)?;

        if a > b {
            return Err(err!("Start date > end date"));
        }
        return Ok((a, b));
    }

    Err(err!("Must give args like 202501, 202501-202506,*-202501"))
}
