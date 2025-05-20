use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::misc::{paths, status_mark};

use super::{OperType, field_style};

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

impl LogEntry {
    pub fn to_log(&self) -> String {
        let status = if self.is_succ {
            //  ✓ 和 ✗
            status_mark::succ()
        } else {
            status_mark::fail()
        };

        // 带空格的，要包裹单引号
        let arg = if self.arg.find(" ").is_some() {
            field_style::grey(&format!("'{}'", self.arg))
        } else {
            self.arg.clone()
        };

        // todo 多行的remark，在另起一行时要添加足够的空格来对其自己开始的位置
        let remark = if self.remark.is_empty() {
            field_style::grey(&"(no remark)".to_string())
        } else {
            field_style::grey(&paths::apply_alias(&self.remark))
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
            field_style::grey(&self.time),
            status,
            self.oper.to_padded_str(),
            arg,
            remark,
            id,
        )
    }
}
