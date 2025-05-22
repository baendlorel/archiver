use crate::{err_warn, wrap_err, wrap_result};

use owo_colors::OwoColorize;
use std::{fs, u32};

use super::parse_range;
use crate::misc::paths;
use crate::models::error::ArchiverError;
use crate::models::types::LogEntry;

const MAX_CASUAL_COUNT: usize = 15;

pub fn load(range: &Option<String>) -> Result<(), ArchiverError> {
    // 是否随便看看，如果没有给定range，那么别输出过多条数
    let casual = range.is_none();

    // 考虑到日期本质上是一个不定型进制数，可以考虑直接转为数字来对比大小
    let range = parse_range(range)?;
    let year_range = (range.0 / 100, range.1 / 100);

    let years = paths::get_years_desc();
    let mut logs: Vec<String> = vec![];
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
        wrap_result!(load_from_content(casual, &range, &content, &mut logs))?;

        // 如果没设置范围，只是随便看看日志，那么不要打得太多
        // 同时在load和log_content使用方可生效
        if casual == true && logs.len() >= MAX_CASUAL_COUNT {
            break;
        }
    }

    // 反着加进来的，还得反着输出出去
    logs.iter().rev().for_each(|l| println!("{}", l));

    // 如果是随便看看而且到达最大值，那么提示可以看更多
    if casual && logs.len() == MAX_CASUAL_COUNT {
        println!(
            // "Recent {} logs displayed. Specify a [range] to see more. e.g. arv lg 202505",
            "Recent {} logs displayed.",
            MAX_CASUAL_COUNT
        );
    }

    if logs.len() == 0 {
        println!("No logs found");
    }

    Ok(())
}

fn load_from_content(
    casual: bool,
    parsed_range: &(u32, u32),
    content: &str,
    logs: &mut Vec<String>,
) -> Result<(), ArchiverError> {
    // 把年月取出来组成一个整数用于比较
    let parse_ym = |s: &str| -> Result<u32, ArchiverError> {
        // 现在s基本认为一定是一个时间字符串
        let mut iter = s.split("-");
        let raw_year = iter
            .next()
            .ok_or(err_warn!(format!("Year parse failed for '{}'", s)))?;
        let raw_month = iter
            .next()
            .ok_or(err_warn!(format!("Month parse failed for '{}'", s)))?;

        let year = wrap_err!(raw_year.parse::<u32>())?;
        let month = wrap_err!(raw_month.parse::<u32>())?;

        if month > 12 || month < 1 {
            return err_warn!("Month > 12, parse failed for '{}'", s)));
        }

        Ok(year * 100 + month)
    };

    // 由于最新的日志在最底下一行，所以要倒叙遍历
    // 解析每行JSON
    for line in content.lines().rev().filter(|l| !l.trim().is_empty()) {
        let result = serde_json::from_str::<LogEntry>(line);

        // 这里不需要报错反回去，只需要跳过报错部分，让程序运行更稳定些
        if let Ok(entry) = &result {
            let ym = parse_ym(&entry.time)?;
            if ym < parsed_range.0 || ym > parsed_range.1 {
                continue; // 跳过不在范围内的日期
            }

            logs.push(entry.to_log());
            // 如果没设置范围，只是随便看看日志，那么不要打得太多
            // 同时在load和log_content使用方可生效
            if casual && logs.len() >= MAX_CASUAL_COUNT {
                return Ok(());
            }
        }

        // 注意此处不是ArchiverError，不能用宏
        if let Err(e) = &result {
            println!("{}: {}", "Parse log failed".red(), e.to_string().yellow());
        }
    }

    Ok(())
}
