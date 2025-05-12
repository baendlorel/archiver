use std::{fs::OpenOptions, io::Write, path::PathBuf};

use serde::Serialize;

pub fn append_entry<T>(entry: &T, file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>>
where
    T: ?Sized + Serialize,
{
    // 序列化为JSON
    let json_line = serde_json::to_string(&entry)?;

    // 以追加模式打开文件
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    // 写入
    file.write_all(json_line.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}
