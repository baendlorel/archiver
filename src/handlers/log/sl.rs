use crate::wrap_result;

use chrono::Datelike;
use std::u32;

use super::parser;
use crate::{
    misc::{ForceToString, dt, jsonl, mark, paths},
    models::{
        error::ArchiverError,
        types::{LogEntry, OperType},
    },
};

const MAX_CASUAL_COUNT: usize = 15;

pub fn save(
    oper: &OperType,
    arg: &str,
    is_succ: bool,
    id: Option<u32>,
    remark: Option<String>,
) -> Result<(), ArchiverError> {
    // 获取日志文件路径
    let log_file_path = paths::get_log_path(dt::now_year());

    let normalized_remark = match oper {
        OperType::Put => {
            let full_path = paths::CWD.join(arg);
            if is_succ {
                full_path.force_to_string()
            } else {
                remark.unwrap_or("".to_string())
            }
        }
        _ => remark.unwrap_or("".to_string()),
    };

    // 准备日志内容
    let log_entry = LogEntry {
        time: dt::now_dt(),
        is_succ,
        oper: oper.clone(),
        arg: arg.to_string(),
        remark: normalized_remark,
        id,
    };

    wrap_result!(jsonl::append(&log_entry, &log_file_path))?;
    Ok(())
}

pub fn load(range: &Option<String>) -> Result<(Vec<String>, bool), ArchiverError> {
    // 是否随便看看，如果没有给定range，那么别输出过多条数
    let casual = range.is_none();

    // 考虑到日期本质上是一个不定型进制数，可以考虑直接转为数字来对比大小
    let (a, b) = wrap_result!(parser::normalize_range(range))?;
    let (ya, yb) = (a.year(), b.year());

    let years = paths::get_years_desc();
    let mut logs: Vec<String> = vec![];
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

        for l in cur_logs {
            if l.time < a || l.time > b {
                continue; // 跳过不在范围内的日期
            }
            logs.push(l.to_log());
            // 如果没设置范围，只是随便看看日志，那么不要打得太多
            // 同时在load和log_content使用方可生效
            if casual && logs.len() >= MAX_CASUAL_COUNT {
                break 'year_loop;
            }
        }
    }

    // 反着加进来的，还得反着输出
    // logs.iter().rev().for_each(|l| println!("{}", l));

    // 如果是随便看看而且到达最大值，那么提示可以看更多
    let reach_casual_limit = casual && logs.len() == MAX_CASUAL_COUNT;
    // if casual && logs.len() == MAX_CASUAL_COUNT {
    //     println!("Recent {} logs displayed.", MAX_CASUAL_COUNT);
    // }

    if logs.len() == 0 {
        println!("No logs found");
    }

    Ok((logs, reach_casual_limit))
}
