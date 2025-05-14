/// 操作日志加载错误枚举
#[derive(PartialEq, Debug)]
pub enum ConfigError {
    /// 文件读取/写入错误
    IoError(String),

    EmptyName(String),

    InvalidAliasEntryForm(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let description = match self {
            ConfigError::IoError(m) => format!("IoError: {}", m),
            ConfigError::EmptyName(m) => format!("EmptyName: {}", m),
            ConfigError::InvalidAliasEntryForm(m) => format!("InvalidAliasEntryForm: {}", m),
        };
        f.write_str(description.as_str())
    }
}

impl std::error::Error for ConfigError {}

/// 转换标准 IO 错误到自定义错误
impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::IoError(error.to_string())
    }
}
