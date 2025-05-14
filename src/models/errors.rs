mod archive_error;
mod config_error;
mod list_error;
mod oper_log_error;
mod restore_error;
mod with_backtrace;
// TODO 规整重复的错误内容，最好能继承
// TODO backtrace
pub use archive_error::ArchiveError;
pub use config_error::ConfigError;
pub use list_error::ListError;
pub use oper_log_error::OperLogError;
pub use restore_error::RestoreError;
use with_backtrace::WithBacktrace;
