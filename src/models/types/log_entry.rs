use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use super::{OperType, field_style};

/// 定义用于序列化到JSON的日志条目结构
#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    pub time: String,    // 操作时间
    pub status: String,  // 是否成功
    pub oper: OperType,  // 操作类型
    pub arg: String,     // 操作参数
    pub remark: String,  // 备注
    pub id: Option<u32>, // archive id，如果有的话
}

impl LogEntry {
    pub fn to_log(&self) -> String {
        let status = if self.status == "succ" {
            "succ".green().to_string()
            // "✅"
        } else {
            "fail".red().to_string()
            // "❌"
        };

        let arg = if self.arg.is_empty() {
            field_style::grey(&"(empty)".to_string())
        } else {
            self.arg.clone()
        };

        let id = if let Some(id) = self.id {
            if self.oper == OperType::Archive {
                String::from("-> ") + &field_style::id_to_str(id)
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
            field_style::cwd(&self.remark),
            id,
        )
    }
}
