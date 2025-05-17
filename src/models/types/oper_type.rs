use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

/// 操作类型枚举
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OperType {
    Put,
    Restore,
    Config { option: &'static str },
}

/// 为OperationType实现转换为字符串的方法
impl OperType {
    pub fn to_padded_str(&self) -> String {
        match self {
            OperType::Put => "put".yellow().to_string(),
            OperType::Restore => "rst".cyan().to_string(),
            OperType::Config { option } => format!("{} {}", "cfg".bright_purple(), option),
        }
    }
}
