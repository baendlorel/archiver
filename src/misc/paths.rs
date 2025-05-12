use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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
            m.insert(right.to_string(), left.to_string());
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

pub fn alias_path(path_str: String) -> String {
    ALIAS_MAP.iter().for_each(|(origin, alias)| {
        let path_str = path_str.trim();
        // 检查路径是否以 home 目录开头
        if path_str.starts_with(origin) {
            // 替换 home 目录为波浪线
            let relative_path = &path_str[origin.len()..];
            // 处理可能的路径分隔符
            let relative_path = relative_path.trim_start_matches('/');
            let path_buf = PathBuf::from(alias);
            path_buf.join(relative_path).to_string_lossy().to_string()
        } else {
            // 不是 home 目录下的路径，保持不变
            path_str.to_string()
        }
    });
}
