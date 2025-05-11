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
    pub fn to_padded_str(&self) -> &'static str {
        match self {
            OperType::Archive => "Archive",
            OperType::Restore => "Restore",
            OperType::List => "List   ",
            OperType::Log => "Log    ",
        }
    }
}
