use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::cli::short;
use crate::core::vault;
use crate::misc::{dt, mark, paths};
use crate::models::serde_custom::{boolean, naive_date_time};
use crate::models::types::field_style::CustomColors;

use super::Operation;

/// 定义用于序列化到JSON的日志条目结构
#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    #[serde(rename = "oat", with = "naive_date_time")]
    pub opered_at: NaiveDateTime, // 操作时间

    #[serde(rename = "s", with = "boolean")]
    pub is_succ: bool, // 是否成功

    #[serde(rename = "o")]
    pub oper: Operation, // 操作类型

    #[serde(rename = "r")]
    pub remark: String, // 备注

    #[serde(rename = "aid")]
    pub archive_id: Option<u32>, // archive id，如果有的话

    #[serde(rename = "vid")]
    pub vault_id: Option<u32>, // archive id，如果有的话
}

impl LogEntry {
    // todo 对于可以多重输入的命令的日志，改由处理函数返回LogEntry数组，然后外部println
    /// 创建一个状态为succ的日志条目
    // pub fn succ() -> Self {}

    /// 创建一个状态为fail的日志条目
    // pub fn fail() -> Self {}

    pub fn to_display(&self) -> String {
        let time = dt::to_dt_string(&self.opered_at);

        let status = if self.is_succ {
            //  ✓ 和 ✗
            mark::succ()
        } else {
            mark::fail()
        };

        let archive_id = if let Some(archive_id) = self.archive_id {
            if self.oper.main == short::main::PUT {
                archive_id.to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let vault_name = if let Some(vault_id) = self.vault_id {
            match self.oper.main.as_str() {
                short::main::PUT => vault::get_name(vault_id),
                short::main::VAULT => vault::get_name(vault_id),
                _ => String::new(),
            }
        } else {
            String::new()
        };

        // let remark = if self.remark.is_empty() {
        //     if vault_name.is_empty() && archive_id.is_empty() {
        //         "(no remark)".to_string()
        //     } else {
        //         String::new()
        //     }
        // } else {
        //     let r = paths::apply_alias(&self.remark);
        //     r.replace("\n", "\\n").to_string()
        // };

        let remark = paths::apply_alias(&self.remark)
            .replace("\n", "\\n")
            .to_string();

        let avid = match (archive_id.is_empty(), vault_name.is_empty()) {
            (true, true) => String::new(),
            (false, true) => format!("(aid:{})", archive_id.magenta()),
            (true, false) => format!("(vlt:{})", vault_name.blue()),
            (false, false) => format!("(aid:{}, vlt:{})", archive_id.magenta(), vault_name.blue()),
        };

        // 下面处理remark、archive_id和vault_name的显示
        let rav = match (self.remark.is_empty(), avid.is_empty()) {
            (true, true) => "(no remark)".bright_black().to_string(),
            (false, true) => remark.bright_black().to_string(),
            (true, false) => avid,
            (false, false) => format!("{} {}", remark.bright_black(), avid),
            _ => String::new(),
        };

        format!(
            "{} {} - {} - {}",
            time.bright_black(),
            status,
            self.oper.to_display(),
            rav,
        )
    }
}
