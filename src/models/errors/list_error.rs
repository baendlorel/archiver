use super::with_backtrace::WithBacktrace;
use crate::impl_from_with_backtrace;

impl_from_with_backtrace!(std::io::Error, ListError::IoError);
impl_from_with_backtrace!(serde_json::Error, ListError::JsonParse);

/// 操作日志加载错误枚举
#[derive(thiserror::Error, Debug)]
pub enum ListError {
    #[error("IoError: {0}")]
    IoError(WithBacktrace<std::io::Error>),

    #[error("JsonParseError: {0}")]
    JsonParse(WithBacktrace<serde_json::Error>),

    /// 归档目标没有找到
    #[error("TargetNotFound: {0}")]
    TargetNotFound(WithBacktrace<String>),
}
