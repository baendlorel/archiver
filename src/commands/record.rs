pub mod oper_log {
    use crate::misc::constants::paths;
    use chrono::{Datelike, Local};
    use serde::{Deserialize, Serialize};
    use serde_json;
    use std::fs::{self, OpenOptions};
    use std::io::{self, Write};
    use std::path::Path;

    pub enum Oper {
        Archive,
        Restore,
        List,
        Log,
    }

    // 为Oper实现转换为字符串的方法
    impl Oper {
        pub fn to_str(&self) -> &'static str {
            match self {
                Oper::Archive => "Archive",
                Oper::Restore => "Restore",
                Oper::List => "List   ",
                Oper::Log => "Log    ",
            }
        }
    }

    /// 定义用于序列化到JSON的日志条目结构
    #[derive(Serialize, Deserialize)]
    struct LogEntry {
        time: String,   // 操作时间
        status: String, // 是否成功
        oper: String,   // 操作类型
        arg: String,    // 操作参数
        id: i64,        // archive id，如果有的话
    }

    /// 保存操作日志
    fn save(oper: Oper, arg: String, is_succ: bool, id: i64) -> Result<(), io::Error> {
        // 获取日志文件路径
        let home = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "无法获取用户主目录"))?;

        let log_file_path = home
            .join(Path::new(paths::ROOT))
            .join(Path::new(paths::OPER_LOG))
            .join(Path::new(format!("{}.jsonl", Local::now().year()).as_str()));

        // 确保日志目录存在
        let log_dir = Path::new(&paths::OPER_LOG);
        if !log_dir.exists() {
            fs::create_dir_all(log_dir)?;
        }

        // 获取当前时间
        let opered_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // 准备日志内容
        let log_entry = LogEntry {
            time: opered_at,
            status: if is_succ { "succ" } else { "fail" }.to_string(),
            oper: oper.to_str().to_string(),
            arg,
            id, // archive id，如果有的话
        };

        // 序列化为JSON
        let json_line = serde_json::to_string(&log_entry)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // 以追加模式打开文件
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)?;

        // 写入日志
        file.write_all(json_line.as_bytes())?;
        file.write_all(b"\n")?;

        println!("操作日志已保存");
        return Ok(());
    }

    fn load(criteria: Option<String>) -> Result<(), io::Error> {
        // 获取日志文件路径
        let home = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "无法获取用户主目录"))?;

        let year = Local::now().year();
        let log_file_path = home
            .join(paths::ROOT)
            .join(paths::OPER_LOG)
            .join(format!("{}.jsonl", year));

        // 如果文件不存在，返回空数组
        if !log_file_path.exists() {
            return Ok(());
        }

        // 读取文件内容
        let content = fs::read_to_string(log_file_path)?;

        // 解析每行JSON
        let mut entries = Vec::new();
        for line in content.lines() {
            if !line.trim().is_empty() {
                match serde_json::from_str::<LogEntry>(line) {
                    Ok(entry) => entries.push(entry),
                    Err(e) => eprintln!("解析日志行失败: {}", e),
                }
            }
        }

        let filtered_entries: Vec<&LogEntry> = entries
            .iter()
            .filter(|entry| {
                // 尝试解析时间字段
                if let Ok(date_time) =
                    chrono::NaiveDateTime::parse_from_str(&entry.time, "%Y-%m-%d %H:%M:%S")
                {
                    // 检查年份是否在2000-2025之间
                    let year = date_time.year();
                    year >= 2000 && year <= 2025
                } else {
                    // 如果解析失败，则跳过该条目
                    eprintln!("无法解析时间: {}", entry.time);
                    false
                }
            })
            .collect();

        println!("加载了 {} 条操作日志", entries.len());

        println!("加载操作日志");
        Ok(())
    }
}

pub mod list {

    /// `object_name`: 归档对象名称，归档的可能是文件或文件夹
    fn save(object_name: String) {
        println!("保存归档列表");
    }

    fn load(name: Option<String>) {
        if let Some(name) = name {
            println!("加载归档列表 {}", name);
        } else {
            println!("加载所有归档列表");
        }
    }
}
