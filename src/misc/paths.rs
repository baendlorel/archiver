use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::{MAIN_SEPARATOR, PathBuf};

use super::force_no_loss;

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

/// 用户文件夹
static HOME_DIR: Lazy<PathBuf> =
    Lazy::new(|| dirs::home_dir().expect("Failed to get home directory"));

/// 当前工作目录
pub static CWD: Lazy<PathBuf> =
    Lazy::new(|| std::env::current_dir().expect("Failed to get current directory"));

/// 程序主目录
pub static ROOT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = HOME_DIR.join(paths::ROOT);
    // 检查路径是否存在，不存在则创建
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create root directory");
    }
    path
});

/// 日志目录
pub static LOGS_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = ROOT_DIR.join(paths::LOGS_DIR);
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create logs_dir directory");
    }
    path
});

/// 配置目录
pub static CONFIGS_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = ROOT_DIR.join(paths::CONFIGS_DIR);
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create configs_dir directory");
    }
    path
});

/// 归档记录文件路径
pub static LIST_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| ROOT_DIR.join(paths::LIST_FILE));

pub static CURRENT_ID: Lazy<u32> = Lazy::new(|| {
    let auto_incr_file = CONFIGS_DIR.join(paths::AUTO_INCR_FILE);
    if !auto_incr_file.exists() {
        fs::write(&auto_incr_file, "1").expect("Failed to create auto increment file");
        return 1;
    }

    fs::read_to_string(&auto_incr_file)
        .expect("Failed to read auto increment file")
        .trim()
        .parse::<u32>()
        .expect("Failed to parse auto increment value")
});

/// 别名映射表
static ALIAS_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let dir_alias_file = CONFIGS_DIR.join(paths::DIR_ALIAS_FILE);
    let mut m = HashMap::new();

    m.insert("~".to_string(), force_no_loss(HOME_DIR.as_os_str()));

    if !dir_alias_file.exists() {
        return m;
    }

    let content = fs::read_to_string(dir_alias_file).expect("Cannot read dir_alias_file ");

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue; // 跳过空行
        }

        // 只看能分出结果的
        if let Some(tuple) = line.split_once("=") {
            let left = tuple.0.trim();
            let right = tuple.1.trim();
            if left.is_empty() || right.is_empty() {
                continue; // 跳过空行
            }

            // 期望的格式是 ~=/home/xxx , @projects=/home/xxx/projects 这样风格的
            m.insert(left.to_string(), right.to_string());
        }
    }

    m
});

pub fn auto_incr_id() -> u32 {
    let auto_incr_file = CONFIGS_DIR.join(paths::AUTO_INCR_FILE);
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

pub fn apply_alias(path_str: String) -> String {
    // 使用普通循环，可以在找到匹配时提前返回
    for (alias, origin) in ALIAS_MAP.iter() {
        // println!("origin:{} alias:{} path_str:{}", origin, alias, path_str);
        if path_str.starts_with(origin) {
            // 替换原始路径前缀为别名
            let relative_path = &path_str[origin.len()..];
            // 处理可能的路径分隔符
            let relative_path = relative_path.trim_start_matches(MAIN_SEPARATOR);

            // 构建新路径
            return format!("{}{}{}", alias, MAIN_SEPARATOR, relative_path);
        }
    }

    path_str
}

pub fn get_log_path(year: i32) -> PathBuf {
    LOGS_DIR.join(format!("{}.jsonl", year))
}

pub fn get_all_logs_year() -> Vec<i32> {
    let mut logs = Vec::new();
    if let Ok(entries) = fs::read_dir(&*LOGS_DIR) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.path().file_name() {
                // logs下的文件一定是年份为名字，直接转换
                if let Ok(year) = file_name
                    .to_string_lossy()
                    .to_string()
                    .trim_end_matches(".jsonl")
                    .parse::<i32>()
                {
                    logs.push(year);
                }
            }
        }
    }
    logs
}
