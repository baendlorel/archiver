pub mod oper_log {
    use crate::misc::constants::paths;
    use chrono::{Datelike, Local};
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
                Oper::List => "List",
                Oper::Log => "Log",
            }
        }
    }

    pub struct OperLogParams {
        pub oper: Oper,
        pub arg: String,
        pub id: i64,
        pub is_succ: bool,
    }

    fn save(params: OperLogParams) -> Result<(), io::Error> {
        // 获取日志文件路径
        let home = match dirs::home_dir() {
            Some(path) => path,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "无法获取用户主目录",
                ));
            }
        };

        let log_file_path = home
            .join(Path::new(paths::ROOT))
            .join(Path::new(paths::OPER_LOG))
            .join(Path::new(format!("{}.log", Local::now().year()).as_str()));

        // 确保日志目录存在
        let log_dir = Path::new(&paths::OPER_LOG);
        if !log_dir.exists() {
            fs::create_dir_all(log_dir)?;
        }

        // 获取当前时间
        let opered_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // 准备日志内容
        let log_entry = format!(
            "{} | {} | {} | {}\n",
            opered_at,
            params.oper.to_str(),
            params.arg,
            if params.is_succ { 1 } else { 0 }
        );

        // 以追加模式打开文件
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)?;

        // 写入日志内容
        match file.write_all(log_entry.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
            }
        };

        println!("操作日志已保存");
        return Ok(());
    }

    fn load() {
        println!("加载操作日志");
    }
}

pub mod list {
    pub struct ListItemParams {
        pub name: String,
        pub is_dir: bool,
        pub dir: String,
    }

    fn save(args: ListItemParams) {
        println!("保存归档列表");
    }

    fn load(name: Option<String>) {
        println!("加载归档列表");
    }
}
