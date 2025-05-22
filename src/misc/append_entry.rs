use std::{fs::OpenOptions, io::Write, path::PathBuf};

use serde::Serialize;

use crate::{models::error::ArchiverError, wrap_err_fatal};

/// 给jsonl文件追加一行
pub fn append_entry<T>(entry: &T, file_path: PathBuf) -> Result<(), ArchiverError>
where
    T: ?Sized + Serialize,
{
    // 序列化为JSON
    let json_line = wrap_err_fatal!(serde_json::to_string(&entry))?;

    // 以追加模式打开文件
    let mut file = wrap_err_fatal!(OpenOptions::new().create(true).append(true).open(file_path))?;

    // 写入
    wrap_err_fatal!(file.write_all(json_line.as_bytes()))?;
    wrap_err_fatal!(file.write_all(b"\n"))?;

    Ok(())
}
