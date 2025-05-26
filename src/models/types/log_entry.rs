use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::cli::short;
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

/// 为remark换行的缩进准备的常量
/// 由此公式算得：字段间空格数量+状态字符数量+短横线两个
/// 当前为 5+1+3
// const INVARIANT_PADDING: usize = 9;
impl LogEntry {
    // todo 对于可以多重输入的命令的日志，改由处理函数返回LogEntry数组，然后外部println
    /// 创建一个状态为succ的日志条目
    // pub fn succ() -> Self {}

    /// 创建一个状态为fail的日志条目
    // pub fn fail() -> Self {}

    pub fn to_log(&self) -> String {
        let time = dt::to_dt_string(&self.opered_at);

        let status = if self.is_succ {
            //  ✓ 和 ✗
            mark::succ()
        } else {
            mark::fail()
        };

        let remark = if self.remark.is_empty() {
            "(no remark)".grey()
        } else {
            let r = paths::apply_alias(&self.remark);
            // let padding_count = time.len() + INVARIANT_PADDING + self.oper.len() + self.arg.len();
            // let replacer = format!(
            //     "\n{}{}{}{}{}{}",
            //     "t".repeat(self.time.len()),
            //     "-".repeat(5),
            //     "o".repeat(self.oper.len()),
            //     " ".repeat(1),
            //     "a".repeat(self.arg.len()),
            //     " ".repeat(3),
            // );
            // let replacer = format!("\n{}", " ".repeat(padding_count));
            // r.replace("\n", replacer.as_str()).grey()
            r.replace("\n", "\\n").grey()
        };

        let archive_id = if let Some(archive_id) = self.archive_id {
            if self.oper.main == short::main::PUT {
                format!("-> {}", archive_id.magenta())
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let vault_id = if let Some(vault_id) = self.vault_id {
            match self.oper.main.as_str() {
                short::main::PUT => format!("(vlt:{})", vault_id.bright_blue()),
                short::main::VAULT => format!("(vlt:{})", vault_id.bright_blue()),
                _ => String::new(),
            }
        } else {
            String::new()
        };

        format!(
            "{} {} - {} - {} {}{}",
            time.grey(),
            status,
            self.oper.to_string(),
            remark,
            archive_id,
            vault_id,
        )
    }
}
