use crate::wrap_result;

use chrono::{Datelike, Local};
use std::u32;

use crate::misc::{ForceToString, append_entry, paths};
use crate::models::error::ArchiverError;
use crate::models::types::{LogEntry, OperType};

pub fn save(
    oper: &OperType,
    arg: &str,
    is_succ: bool,
    id: Option<u32>,
    remark: Option<String>,
) -> Result<(), ArchiverError> {
    // 获取日志文件路径
    let log_file_path = paths::get_log_path(Local::now().year() as u32);

    // 确保日志目录存在
    // 获取当前时间
    let opered_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let normalized_remark = match oper {
        OperType::Put => {
            let full_path = paths::CWD.join(arg);
            full_path.force_to_string()
        }
        _ => remark.unwrap_or("".to_string()),
    };

    // 准备日志内容
    let log_entry = LogEntry {
        time: opered_at,
        is_succ,
        oper: oper.clone(),
        arg: arg.to_string(),
        remark: normalized_remark,
        id,
    };

    wrap_result!(append_entry(&log_entry, log_file_path))?;
    Ok(())
}
