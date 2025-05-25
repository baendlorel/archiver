use crate::as_fatal;

use serde::{Serialize, de::DeserializeOwned};
use std::{fs, io::Write, path::Path};

use crate::models::{error::ArchiverError, serde_custom::SerdeJson};

/// 给jsonl文件末尾追加一行
pub fn append<T>(entry: &T, file_path: &Path) -> Result<(), ArchiverError>
where
    T: ?Sized + Serialize + DeserializeOwned + SerdeJson,
{
    // 序列化为JSON
    let json_line = as_fatal!(entry.to_json_line())?;

    // 以追加模式打开文件
    let mut file = as_fatal!(
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
    )?;

    // 写入
    as_fatal!(file.write_all(format!("{}\n", json_line).as_bytes()))?;
    Ok(())
}

/// 从jsonl文件加载列表
pub fn load<T>(file_path: &Path) -> Result<Vec<T>, ArchiverError>
where
    T: ?Sized + Serialize + DeserializeOwned + SerdeJson,
{
    if !file_path.exists() {
        return Ok(vec![]); // 如果文件不存在，返回空列表
    }

    // 读取文件内容
    let content = as_fatal!(fs::read_to_string(file_path))?;

    let mut list: Vec<T> = vec![];
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue; // 跳过空行
        }

        match serde_json::from_str::<T>(line) {
            Ok(entry) => list.push(entry),
            Err(e) => {
                println!("Failed to parse line as JSON: {}", e)
            }
        }
    }

    Ok(list)
}

/// 保存列表到jsonl文件
pub fn save<T>(list: &[T], file_path: &Path) -> Result<(), ArchiverError>
where
    T: ?Sized + Serialize + DeserializeOwned + SerdeJson,
{
    let mut content: Vec<String> = vec![];
    for entry in list {
        let l = as_fatal!(entry.to_json_line())?;
        content.push(l);
    }

    as_fatal!(fs::write(
        file_path,
        format!("{}\n", content.join("\n")).as_bytes()
    ))?;

    Ok(())
}
