/// 操作日志加载错误枚举
#[derive(PartialEq, Debug)]
pub enum ListError {
    /// 归档目标没有找到
    TargetNotFound(String),

    /// 文件读取/写入错误
    IoError(String),

    /// JSON 解析错误
    JsonParseError(String),
}

impl std::fmt::Display for ListError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let description = match self {
            ListError::TargetNotFound(m) => format!("TargetNotFound: {}", m),
            ListError::IoError(m) => format!("IoError: {}", m),
            ListError::JsonParseError(m) => {
                format!("JsonParseError: {}", m)
            }
        };
        f.write_str(description.as_str())
    }
}

impl std::error::Error for ListError {}

/// 转换标准 IO 错误到自定义错误
impl From<std::io::Error> for ListError {
    fn from(error: std::io::Error) -> Self {
        ListError::IoError(error.to_string())
    }
}

/// 转换序列化错误到自定义错误  
impl From<serde_json::Error> for ListError {
    fn from(error: serde_json::Error) -> Self {
        ListError::JsonParseError(error.to_string())
    }
}
