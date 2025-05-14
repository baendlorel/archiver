use super::ListError;

/// 操作日志加载错误枚举
#[derive(PartialEq, Debug)]
pub enum RestoreError {
    IoError(String),

    // 归档文件未找到
    ArchivedFileMissing(String),

    // 原目录下有了另一个同名文件/文件夹
    DuplicatedOrigin(String),

    MarkAsRestoredFail(String),

    /// 归档目标已经恢复过了
    AlreadyRestored(String),
}

impl std::fmt::Display for RestoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let description = match self {
            RestoreError::IoError(m) => format!("IoError: {}", m),
            RestoreError::ArchivedFileMissing(m) => format!("ArchivedFileMissing: {}", m),
            RestoreError::DuplicatedOrigin(m) => format!("DuplicatedOrigin: {}", m),
            RestoreError::MarkAsRestoredFail(m) => format!("MarkAsRestoredFail: {}", m),
            RestoreError::AlreadyRestored(m) => format!("AlreadyRestored: {}", m),
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
