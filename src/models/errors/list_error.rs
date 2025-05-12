use std::fmt;
use std::io;

/// 操作日志加载错误枚举
#[derive(PartialEq, Debug)]
pub enum ListError {
    TargetNotFound(String),

    /// 文件读取/写入错误
    IoError(String),

    /// JSON 解析错误
    JsonParseError(String),
}

impl fmt::Display for ListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            ListError::TargetNotFound(m) => format!("List::TargetNotFound: {}", m),
            ListError::IoError(m) => format!("List::IoError: {}", m),
            ListError::JsonParseError(m) => {
                format!("List::JsonParseError: {}", m)
            }
        };
        f.write_str(description.as_str())
    }
}

impl std::error::Error for ListError {}

/// 转换标准 IO 错误到自定义错误
impl From<io::Error> for ListError {
    fn from(error: io::Error) -> Self {
        ListError::IoError(error.to_string())
    }
}

/// 转换序列化错误到自定义错误  
impl From<serde_json::Error> for ListError {
    fn from(error: serde_json::Error) -> Self {
        ListError::JsonParseError(error.to_string())
    }
}
