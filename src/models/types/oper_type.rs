use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

/// 操作类型枚举
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OperType {
    Put,
    Restore,
    Config { option: String },
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
    pub fn len(&self) -> usize {
        match self {
            OperType::Put => 3,
            OperType::Restore => 3,
            OperType::Config { option } => 3 + 1 + option.len(), // e.g. cfg_alias.add
        }
    }
}
