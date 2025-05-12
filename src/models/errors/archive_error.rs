use std::fmt;

use super::ListError;
use super::OperLogError;

/// 操作日志加载错误枚举
#[derive(PartialEq, Debug)]
pub enum ArchiveError {
    /// 归档目标不存在
    TargetNotFound(String),
    InvalidTarget(String),
    IoError(String),

    ListSaveError {
        source: ListError,
    },
    LogSaveError {
        source: OperLogError,
    },
}

impl fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            ArchiveError::IoError(m) => format!("IoError: {}", m),
            ArchiveError::TargetNotFound(m) => format!("TargetNotFound: {}", m),
            ArchiveError::InvalidTarget(m) => format!("InvalidTarget: {}", m),
            // 外部错误
            ArchiveError::ListSaveError { source } => {
                format!("ListSaveError: {}", source)
            }
            ArchiveError::LogSaveError { source } => {
                format!("LogSaveError: {}", source)
            }
        };
        f.write_str(description.as_str())
    }
}

impl std::error::Error for ArchiveError {}

impl From<std::io::Error> for ArchiveError {
    fn from(error: std::io::Error) -> Self {
        ArchiveError::IoError(error.to_string())
    }
}

impl From<ListError> for ArchiveError {
    fn from(error: ListError) -> Self {
        ArchiveError::ListSaveError { source: error }
    }
}

impl From<OperLogError> for ArchiveError {
    fn from(error: OperLogError) -> Self {
        ArchiveError::LogSaveError { source: error }
    }
}
