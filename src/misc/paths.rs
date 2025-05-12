use lazy_static::lazy_static;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

mod paths {
    // 目录
    pub const ROOT: &str = ".archiver";
    pub const LOGS_DIR: &str = "logs";
    pub const CONFIGS_DIR: &str = "configs";

    // 特定文件
    pub const LIST_FILE: &str = "archive-list.jsonl";
    pub const AUTO_INCR_FILE: &str = "auto-incr";
    pub const DIR_ALIAS_FILE: &str = "dir-alias";
}

lazy_static! {
    pub static ref ALIAS_MAP: Mutex<HashMap<String, u32>> = {
        let mut m = HashMap::new();
        m.insert("init".to_string(), 1);
        Mutex::new(m)
    };

    pub static ref ROOT_DIR:PathBuf = {
        let mut path = dirs::home_dir().expect("Failed to get home directory");
        path.push(paths::ROOT);
        // 检查路径是否存在，不存在则创建
        if !path.exists() {
            fs::create_dir_all(&path).expect("Failed to create root directory");
        }
        path
    };
}

pub fn root_dir() -> PathBuf {
    let mut path = dirs::home_dir().expect("Failed to get home directory");
    path.push(ROOT);
    // 检查路径是否存在，不存在则创建
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create root directory");
    }
    path
}

pub fn logs_dir() -> PathBuf {
    let path = root_dir().join(paths::LOGS_DIR);
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create logs_dir directory");
    }
    path
}

pub fn configs_dir() -> PathBuf {
    let path = root_dir().join(CONFIGS_DIR);
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create configs_dir directory");
    }
    path
}

pub fn list_file_path() -> PathBuf {
    let path = root_dir();
    path.join(LIST_FILE)
}

pub fn auto_incr_id() -> u32 {
    let auto_incr_file = configs_dir().join(AUTO_INCR_FILE);
    if !auto_incr_file.exists() {
        fs::write(&auto_incr_file, "1").expect("Failed to create auto increment file");
        return 1;
    }
    let content = fs::read_to_string(&auto_incr_file).expect("Failed to read auto increment file");
    let new_id = 1 + content
        .trim()
        .parse::<u32>()
        .expect("Failed to parse auto increment value");
    fs::write(&auto_incr_file, new_id.to_string()).expect("Failed to create auto increment file");

    new_id
}

pub fn cwd() -> PathBuf {
    std::env::current_dir().expect("Failed to get current directory")
}

pub fn self_check() {
    cwd();
}

pub fn alias_path(path_str: String, alias: String) -> String {
    // 检查路径是否以 home 目录开头
    if path_str.starts_with(&alias) {
        // 替换 home 目录为波浪线
        let relative_path = &path_str[alias.len()..];
        // 处理可能的路径分隔符
        let relative_path = relative_path.trim_start_matches('/');
        let path_buf = PathBuf::from(alias);
        path_buf.join(relative_path).to_string_lossy().to_string()
    } else {
        // 不是 home 目录下的路径，保持不变
        path_str.to_string()
    }
}
