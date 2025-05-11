use serde::{Deserialize, Serialize};

use super::OperType;

/// 定义用于序列化到JSON的日志条目结构
#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    pub time: String,    // 操作时间
    pub status: String,  // 是否成功
    pub oper: OperType,  // 操作类型
    pub arg: String,     // 操作参数
    pub id: Option<u32>, // archive id，如果有的话
}

impl LogEntry {
    pub fn to_str(&self) -> String {
        format!(
            "{} - {} - {} - {} {}",
            self.time,
            self.oper.to_padded_str(),
            self.status,
            self.arg,
            if let Some(id) = self.id {
                format!("(id: {})", id)
            } else {
                "".to_string()
            }
        )
    }
}
