use super::with_backtrace::WithBacktrace;
use crate::impl_from_with_backtrace;

impl_from_with_backtrace!(std::io::Error, OperLogError::IoError);
impl_from_with_backtrace!(serde_json::Error, OperLogError::JsonParse);

#[derive(thiserror::Error, Debug)]
pub enum OperLogError {
    #[error("IoError: {0}")]
    IoError(WithBacktrace<std::io::Error>),

    #[error("JsonParseError: {0}")]
    JsonParse(WithBacktrace<serde_json::Error>),

    #[error("DateParseError: {0}")]
    DateParseError(WithBacktrace<String>),
}
