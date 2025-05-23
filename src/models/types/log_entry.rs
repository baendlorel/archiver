use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::{
    misc::{mark, paths},
    models::types::field_style::Grey,
};

use super::OperType;

/// 定义用于序列化到JSON的日志条目结构
#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    pub time: String,    // 操作时间
    pub is_succ: bool,   // 是否成功
    pub oper: OperType,  // 操作类型
    pub arg: String,     // 操作参数
    pub remark: String,  // 备注
    pub id: Option<u32>, // archive id，如果有的话
}

/// 为remark换行的缩进准备的常量
/// 由此公式算得：字段间空格数量+状态字符数量+短横线两个
/// 当前为 5+1+3
const INVARIANT_PADDING: usize = 9;
impl LogEntry {
    pub fn to_log(&self) -> String {
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
            let padding_count =
                self.time.len() + INVARIANT_PADDING + self.oper.len() + self.arg.len();
            // let replacer = format!(
            //     "\n{}{}{}{}{}{}",
            //     "t".repeat(self.time.len()),
            //     "-".repeat(5),
            //     "o".repeat(self.oper.len()),
            //     " ".repeat(1),
            //     "a".repeat(self.arg.len()),
            //     " ".repeat(3),
            // );
            let replacer = format!("\n{}", " ".repeat(padding_count));
            r.replace("\n", replacer.as_str()).grey()
        };

        let id = if let Some(id) = self.id {
            if self.oper == OperType::Put {
                format!("-> {}", id.magenta())
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        format!(
            "{} {} - {} {} - {} {}",
            self.time.grey(),
            status,
            self.oper.to_padded_str(),
            self.arg,
            remark,
            id,
        )
    }
}
