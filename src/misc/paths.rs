use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::{MAIN_SEPARATOR, PathBuf};

use crate::models::types::ArchiverConfig;
use crate::wrap_expect;

use super::ForceToString;

mod paths {
    // 目录
    pub const ROOT: &str = ".archiver";
    pub const LOGS_DIR: &str = "logs";
    pub const CORE_DIR: &str = "core";

    // 特定文件
    pub const LIST_FILE: &str = "list.jsonl";
    pub const AUTO_INCR_FILE: &str = "auto-incr";
    pub const CONFIG_FILE: &str = "config.json";
}

/// 用户文件夹
pub static HOME_DIR: Lazy<PathBuf> =
    Lazy::new(|| wrap_expect!(dirs::home_dir(), "Failed to get home directory"));

/// 当前工作目录
pub static CWD: Lazy<PathBuf> =
    Lazy::new(|| wrap_expect!(std::env::current_dir(), "Failed to get current directory"));

/// 程序主目录
pub static ROOT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = HOME_DIR.join(paths::ROOT);
    // 检查路径是否存在，不存在则创建
    if !path.exists() {
        wrap_expect!(fs::create_dir_all(&path), "Failed to create root directory");
    }
    path
});

/// 日志目录
pub static LOGS_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = ROOT_DIR.join(paths::LOGS_DIR);
    if !path.exists() {
        wrap_expect!(
            fs::create_dir_all(&path),
            "Failed to create logs_dir directory"
        );
    }
    path
});

/// 配置目录
pub static CORE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = ROOT_DIR.join(paths::CORE_DIR);
    if !path.exists() {
        wrap_expect!(
            fs::create_dir_all(&path),
            "Failed to create core_dir directory"
        );
    }
    path
});

pub static CONFIG_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = CORE_DIR.join(paths::CONFIG_FILE);
    if !path.exists() {
        // let configs = r#"{"alias_list": []}"#;
        let configs = ArchiverConfig { alias_list: vec![] };
        let content = wrap_expect!(
            serde_json::to_string_pretty(&configs),
            "Failed to serialize config"
        );
        wrap_expect!(fs::write(&path, content), "Failed to create config file");
    }
    path
});

/// 归档记录文件路径
pub static LIST_FILE_PATH: Lazy<PathBuf> =
    Lazy::new(|| ROOT_DIR.join(paths::CORE_DIR).join(paths::LIST_FILE));

pub static CURRENT_ID: Lazy<u32> = Lazy::new(|| {
    let auto_incr_file = CORE_DIR.join(paths::AUTO_INCR_FILE);
    if !auto_incr_file.exists() {
        wrap_expect!(
            fs::write(&auto_incr_file, "1"),
            "Failed to create auto increment file"
        );
        return 1;
    }

    let content: String = wrap_expect!(
        fs::read_to_string(&auto_incr_file),
        "Failed to read auto increment file"
    );

    let current_id = wrap_expect!(
        content.trim().parse::<u32>(),
        "Failed to parse auto increment value"
    );

    current_id
});

/// 别名映射表
static ALIAS_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let content = wrap_expect!(
        fs::read_to_string(CONFIG_FILE_PATH.clone()),
        "Cannot read config file"
    );

    let configs = wrap_expect!(
        serde_json::from_str::<ArchiverConfig>(&content),
        "Cannot parse config file"
    );

    let mut map: HashMap<String, String> = HashMap::new();

    map.insert("~".to_string(), HOME_DIR.force_to_string());
    for line in configs.alias_list {
        map.insert(line.alias, line.origin);
    }

    map
});

pub fn auto_incr_id() -> u32 {
    let auto_incr_file = CORE_DIR.join(paths::AUTO_INCR_FILE);
    if !auto_incr_file.exists() {
        wrap_expect!(
            fs::write(&auto_incr_file, "1"),
            "Failed to create auto increment file"
        );
        return 1;
    }
    let content = wrap_expect!(
        fs::read_to_string(&auto_incr_file),
        "Failed to read auto increment file"
    );

    // todo 这样使用貌似是有异常的，运行后查看详情
    let current_id = wrap_expect!(
        content.trim().parse::<u32>(),
        "Failed to parse auto increment value"
    );

    let new_id = 1 + current_id;
    wrap_expect!(
        fs::write(&auto_incr_file, new_id.to_string()),
        "Failed to create auto increment file"
    );

    new_id
}

pub fn apply_alias(path_str: &String) -> String {
    // 使用普通循环，可以在找到匹配时提前返回
    for (alias, origin) in ALIAS_MAP.iter() {
        if path_str.starts_with(origin) {
            // 替换原始路径前缀为别名
            let relative_path = &path_str[origin.len()..];
            // 处理可能的路径分隔符
            let relative_path = relative_path.trim_start_matches(MAIN_SEPARATOR);

            // 构建新路径
            return format!("{}{}{}", alias, MAIN_SEPARATOR, relative_path);
        }
    }

    path_str.clone()
}

pub fn get_log_file_path(year: u32) -> PathBuf {
    LOGS_DIR.join(format!("{}.jsonl", year))
}

pub fn get_all_logs_year() -> Vec<u32> {
    let mut logs = Vec::new();
    if let Ok(entries) = fs::read_dir(&*LOGS_DIR) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.path().file_name() {
                // logs下的文件一定是年份为名字，直接转换
                if let Ok(year) = file_name
                    .to_string_lossy()
                    .to_string()
                    .trim_end_matches(".jsonl")
                    .parse::<u32>()
                {
                    logs.push(year);
                }
            }
        }
    }
    logs
}
