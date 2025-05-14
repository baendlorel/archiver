use super::with_backtrace::WithBacktrace;
use crate::impl_from_with_backtrace;

impl_from_with_backtrace!(std::io::Error, RestoreError::IoError);

#[derive(thiserror::Error, Debug)]
pub enum RestoreError {
    #[error("IoError: {0}")]
    IoError(WithBacktrace<std::io::Error>),

    // 归档文件未找到
    #[error("ArchivedFileMissing: {0}")]
    ArchivedFileMissing(WithBacktrace<String>),

    // 原目录下有了另一个同名文件/文件夹
    #[error("DuplicatedOrigin: {0}")]
    DuplicatedOrigin(WithBacktrace<String>),

    #[error("MarkAsRestoredFail: {0}")]
    MarkAsRestoredFail(WithBacktrace<String>),

    /// 归档目标已经恢复过了
    #[error("AlreadyRestored: {0}")]
    AlreadyRestored(WithBacktrace<String>),
}
