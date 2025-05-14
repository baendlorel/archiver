use super::with_backtrace::WithBacktrace;
use crate::impl_from_with_backtrace;

impl_from_with_backtrace!(std::io::Error, ConfigError::IoError);

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("IoError: {0}")]
    IoError(WithBacktrace<std::io::Error>),

    // 设置alias时涉及的错误
    #[error("AliasAlreadyExists: {0}")]
    AliasAlreadyExists(WithBacktrace<String>),

    #[error("EmptyName: {0}")]
    EmptyName(WithBacktrace<String>),

    #[error("InvalidAliasEntryForm: {0}")]
    InvalidAliasEntryForm(WithBacktrace<String>),

    // 删除alias时涉及的错误
    #[error("AliasNotFound: {0}")]
    AliasNotFound(WithBacktrace<String>),
}
