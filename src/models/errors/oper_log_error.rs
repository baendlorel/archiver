use std::fmt;
use std::io;

/// 操作日志加载错误枚举
#[derive(PartialEq, Debug)]
pub enum OperLogError {
    /// 文件读取/写入错误
    IoError(String),

    /// 日期解析错误
    DateParseError(String),

    /// JSON 解析错误
    JsonParseError(String),
}

impl fmt::Display for OperLogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            OperLogError::DateParseError(m) => {
                format!("OperLog::DateParseError: {}", m)
            }
            OperLogError::IoError(m) => format!("OperLog::IoError: {}", m),
            OperLogError::JsonParseError(m) => {
                format!("OperLog::JsonParseError: {}", m)
            }
        };
        f.write_str(description.as_str())
    }
}

impl std::error::Error for OperLogError {}

/// 转换标准 IO 错误到自定义错误
impl From<io::Error> for OperLogError {
    fn from(error: io::Error) -> Self {
        OperLogError::IoError(error.to_string())
    }
}

/// 转换序列化错误到自定义错误  
impl From<serde_json::Error> for OperLogError {
    fn from(error: serde_json::Error) -> Self {
        OperLogError::JsonParseError(error.to_string())
    }
}

/// 转换时间解析错误到自定义错误
impl From<chrono::ParseError> for OperLogError {
    fn from(error: chrono::ParseError) -> Self {
        OperLogError::DateParseError(error.to_string())
    }
}
