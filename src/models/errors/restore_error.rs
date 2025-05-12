use std::fmt;

use super::ListError;
use super::OperLogError;

/// 操作日志加载错误枚举
#[derive(PartialEq, Debug)]
pub enum RestoreError {
    /// 当前目录无效
    InvalidCwd(String),
    /// 归档目标不存在
    TargetNotFound(String),
    InvalidTarget(String),
    IoError(String),

    // 归档文件未找到
    ArchivedFileMissing(String),

    // 原目录下有了另一个同名文件/文件夹
    DuplicatedOrigin(String),

    MarkAsRestoredFail(String),
}

impl fmt::Display for RestoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            RestoreError::IoError(m) => format!("Restore::IoError: {}", m),
            RestoreError::InvalidCwd(m) => format!("Restore::InvalidCwd: {}", m),
            RestoreError::TargetNotFound(m) => format!("Restore::TargetNotFound: {}", m),
            RestoreError::InvalidTarget(m) => format!("Restore::InvalidTarget: {}", m),
            RestoreError::ArchivedFileMissing(m) => format!("Restore::ArchivedFileMissing: {}", m),
            RestoreError::DuplicatedOrigin(m) => format!("Restore::DuplicatedOrigin: {}", m),
            RestoreError::MarkAsRestoredFail(m) => format!("Restore::MarkAsRestoredFail: {}", m),
        };
        f.write_str(description.as_str())
    }
}

impl std::error::Error for RestoreError {}

impl From<std::io::Error> for RestoreError {
    fn from(error: std::io::Error) -> Self {
        RestoreError::IoError(error.to_string())
    }
}

impl From<ListError> for RestoreError {
    fn from(error: ListError) -> Self {
        RestoreError::MarkAsRestoredFail(error.to_string())
    }
}
