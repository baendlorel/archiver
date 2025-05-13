use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

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
            "S".green().to_string()
        } else {
            "F".red().to_string()
        };

        // TODO 如果参数带有空格，那么用单引号包裹
        let arg = if self.arg.starts_with(" ") || self.arg.ends_with(" ") {
            if self.oper == OperType::Archive {
                if self.arg.len() > 0 {
                    return field_style::grey(&format!("'{}'", self.arg));
                } else {
                    return field_style::grey(&"(no arg)".to_string());
                }
            }
            "".to_string()
        } else {
            self.arg.clone()
        };

        let remark = if self.remark.is_empty() {
            field_style::grey(&"(no remark)".to_string())
        } else {
            field_style::grey(&self.remark)
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
            remark,
            id,
        )
    }
}
