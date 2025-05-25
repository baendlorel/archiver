use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

// todo 感觉这里是重复代码，可能应该impl在archivercommander里面
/// 操作类型枚举
#[derive(Serialize, Deserialize, Clone)]
pub enum OperType {
    Put,     // 支持多个目标
    Restore, // 支持多个ID
    Config(String),
    Vault(String),
    Move,
}

/// 为OperationType实现转换为字符串的方法
impl OperType {
    pub fn to_padded_str(&self) -> String {
        match self {
            OperType::Put => "put".yellow().to_string(),
            OperType::Restore => "rst".cyan().to_string(),
            OperType::Config(action) => format!("{} {}", "cfg".bright_purple(), action),
            OperType::Vault(action) => format!("{} {}", "vlt".bright_purple(), action),
            OperType::Move => "mov".bright_green().to_string(),
        }
    }
}
