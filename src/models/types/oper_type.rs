use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

/// 操作类型枚举
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OperType {
    Archive,
    Restore,
    List,
    Log,
}

/// 为OperationType实现转换为字符串的方法
impl OperType {
    pub fn to_padded_str(&self) -> String {
        match self {
            OperType::Archive => "archive".yellow().to_string(),
            OperType::Restore => "restore".cyan().to_string(),
            OperType::List => "list   ".bright_blue().to_string(),
            OperType::Log => "log    ".bright_magenta().to_string(),
        }
    }
}
