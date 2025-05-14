use super::ListError;
use super::OperLogError;
use super::with_backtrace::WithBacktrace;

use crate::impl_from_with_backtrace;

impl_from_with_backtrace!(std::io::Error, ArchiveError::IoError);

#[derive(thiserror::Error, Debug)]
pub enum ArchiveError {
    #[error("IoError: {0}")]
    IoError(WithBacktrace<std::io::Error>),

    #[error("TargetNotFound: {0}")]
    TargetNotFound(WithBacktrace<String>),

    #[error("InvalidTarget: {0}")]
    InvalidTarget(WithBacktrace<String>),

    #[error("ListSaveError: {0}")]
    ListSaveError(WithBacktrace<ListError>),

    #[error("LogSaveError: {0}")]
    LogSaveError(WithBacktrace<OperLogError>),
}
