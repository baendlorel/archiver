use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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
        let config = ArchiverConfig {
            auto_check_update: "on".to_string(),
            alias: vec![],
        };
        let content = wrap_expect!(
            serde_json::to_string_pretty(&config),
            "Failed to serialize config"
        );
        wrap_expect!(fs::write(&path, content), "Failed to create config file");
    }
    path
});

/// 归档记录文件路径
pub static LIST_FILE_PATH: Lazy<PathBuf> =
    Lazy::new(|| ROOT_DIR.join(paths::CORE_DIR).join(paths::LIST_FILE));

/// 别名映射表
static ALIAS_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let content = wrap_expect!(
        fs::read_to_string(CONFIG_FILE_PATH.clone()),
        "Cannot read config file"
    );

    let config = wrap_expect!(
        serde_json::from_str::<ArchiverConfig>(&content),
        "Cannot parse config file"
    );

    let mut map: HashMap<String, String> = HashMap::new();

    map.insert("~".to_string(), HOME_DIR.force_to_string());
    for line in config.alias {
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

pub fn apply_alias(path_str: &str) -> String {
    // 使用普通循环，可以在找到匹配时提前返回
    // for (alias, origin) in ALIAS_MAP.iter() {
    //     if let Some(idx) = path_str.find(origin) {
    //         // 替换第一个匹配到的 origin 为 alias
    //         let mut replaced = path_str.to_string();
    //         replaced.replace_range(idx..idx + origin.len(), alias);
    //         return replaced;
    //     }
    // }
    for (alias, origin) in ALIAS_MAP.iter() {
        let bytes = path_str.as_bytes();
        let origin_bytes = origin.as_bytes();
        let len = path_str.len();
        let olen = origin.len();

        let mut i = 0;
        while i + olen <= len {
            if &bytes[i..i + olen] == origin_bytes {
                // 判断是不是单词起始
                let is_word_start =
                    i == 0 || !bytes[i - 1].is_ascii_alphanumeric() && bytes[i - 1] != b'_';
                if is_word_start {
                    // 替换第一个匹配到的 origin 为 alias
                    let mut replaced = path_str.to_string();
                    replaced.replace_range(i..i + olen, alias);
                    return replaced;
                }
            }
            i += 1;
        }
    }
    path_str.to_string()
}

pub fn get_log_file_path(year: u32) -> PathBuf {
    LOGS_DIR.join(format!("{}.jsonl", year))
}

/// 获取所有logs文件夹下的日志的年份，从大到小排列
pub fn get_years_desc() -> Vec<u32> {
    let mut years = Vec::new();
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
                    years.push(year);
                }
            }
        }
    }
    years.sort_by(|a, b| b.cmp(a));
    years
}
