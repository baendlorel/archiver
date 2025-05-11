use serde::{Deserialize, Serialize};

/// 操作类型枚举
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OperType {
    Archive,
    Restore,
    List,
    Log,
}

/// 为OperationType实现转换为字符串的方法
impl OperType {
    pub fn to_str(&self) -> &'static str {
        match self {
            OperType::Archive => "Archive",
            OperType::Restore => "Restore",
            OperType::List => "List   ",
            OperType::Log => "Log    ",
        }
    }
}

/// 用于序列化到JSON的日志条目结构
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogEntry {
    pub time: String,   // 操作时间
    pub status: String, // 是否成功
    pub oper: String,   // 操作类型
    pub arg: String,    // 操作参数
    pub id: i64,        // archive id，如果有的话
}
