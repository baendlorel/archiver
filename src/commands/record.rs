pub mod oper_log {
    use crate::misc::paths::paths;
    use chrono::{Datelike, Local};
    use serde::{Deserialize, Serialize};
    use serde_json;
    use std::fs::{self, OpenOptions};
    use std::io::{self, Write};
    use std::path::Path;

    /// 定义用于序列化到JSON的日志条目结构
    #[derive(Serialize, Deserialize)]
    struct LogEntry {
        time: String,   // 操作时间
        status: String, // 是否成功
        oper: String,   // 操作类型
        arg: String,    // 操作参数
        id: i64,        // archive id，如果有的话
    }


pub mod list {

    /// Saves an archive list item
    ///
    /// Adds an object to the archive list, recording information about archived files or directories.
    ///
    /// # Parameters
    ///
    /// * `target` - Archive object name, can be a file or folder
    fn save(target: String) {
        println!("保存归档列表");
    }

    /// Loads the archive list
    ///
    /// Finds specific archive items based on the provided name, or loads all archive items.
    ///
    /// # Parameters
    ///
    /// * `target` - Optional archive name; if provided, loads specific archive items, otherwise loads all
    fn load(target: Option<String>) {
        if let Some(target) = target {
            println!("加载归档列表 {}", target);
        } else {
            println!("加载所有归档列表");
        }
    }
}
