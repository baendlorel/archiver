use std::fmt;
use std::io;

/// 操作日志加载错误枚举
#[derive(PartialEq, Debug)]
pub enum ArchiveError {
    /// 外部错误，可能来自list或log模块
    ExternalError(String),

    /// 当前目录无效
    InvalidCwd(String),

    /// 归档目标不存在
    TargetNotFound(String),
}

impl fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            ArchiveError::ExternalError(m) => m.clone(),
            ArchiveError::InvalidCwd(m) => format!("Archive::InvalidCwd: {}", m),
            ArchiveError::TargetNotFound(m) => format!("Archive::TargetNotFound: {}", m),
        };
        f.write_str(description.as_str())
    }
}

impl std::error::Error for ArchiveError {}
