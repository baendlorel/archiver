use crate::uoe_result;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use super::ForceToString;
use crate::models::{
    json_serde::JsonSerde,
    types::{ArchiverConfig, AutoIncr},
};

mod consts {
    // 目录
    pub const ROOT: &str = ".archiver";
    pub const LOGS_DIR: &str = "logs";
    pub const CORE_DIR: &str = "core";
    pub const VAULTS_DIR: &str = "vaults";
    pub const LIST_DIR: &str = "list";
    pub const DEFAULT_VAULT: &str = "0";

    // 特定文件
    pub const LIST_FILE: &str = "list.jsonl";
    pub const VAULTS_FILE: &str = "vaults.jsonl";
    pub const AUTO_INCR_FILE: &str = "auto-incr.json";
    pub const CONFIG_FILE: &str = "config.json";
}

/// 确保文件夹路径存在的宏，仅在本文件范围使用
macro_rules! ensure_dir {
    ($e:expr) => {
        once_cell::sync::Lazy::new(|| {
            let path = $e;
            if !path.exists() {
                crate::uoe_result!(
                    fs::create_dir_all(&path),
                    format!("Failed to create directory: {}", path.force_to_string())
                );
            }
            path
        })
    };
}

// # 常用路径

/// 用户文件夹
#[cfg(feature = "dev")]
pub static HOME_DIR: Lazy<PathBuf> =
    Lazy::new(|| uoe_result!(std::env::current_dir(), "Failed to get current directory"));

/// 用户文件夹
#[cfg(not(feature = "dev"))]
pub static HOME_DIR: Lazy<PathBuf> =
    Lazy::new(|| crate::uoe_option!(dirs::home_dir(), "Failed to get home directory"));

/// 当前工作目录
pub static CWD: Lazy<PathBuf> =
    Lazy::new(|| uoe_result!(std::env::current_dir(), "Failed to get current directory"));

/// 程序主目录
pub static ROOT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = HOME_DIR.join(consts::ROOT);
    // 检查路径是否存在，不存在则创建
    if !path.exists() {
        uoe_result!(fs::create_dir_all(&path), "Failed to create root directory");
    }
    path
});

/// 日志目录
pub static LOGS_DIR: Lazy<PathBuf> = ensure_dir!(ROOT_DIR.join(consts::LOGS_DIR));

/// 配置目录
pub static CORE_DIR: Lazy<PathBuf> = ensure_dir!(ROOT_DIR.join(consts::CORE_DIR));

/// 归档的文件/文件夹存放的地方
pub static VAULTS_DIR: Lazy<PathBuf> = ensure_dir!(ROOT_DIR.join(consts::VAULTS_DIR));

/// 归档的记录存放的地方
pub static LIST_DIR: Lazy<PathBuf> = ensure_dir!(ROOT_DIR.join(consts::LIST_DIR));

/// 配置文件路径
/// - 该文件存放在CORE_DIR下
/// - 如果文件不存在，则创建一个默认的配置文件。因为配置文件总要读取，必须存在
/// - 如果是目录，则panic
pub static CONFIG_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = CORE_DIR.join(consts::CONFIG_FILE);
    // 从CORE_DIR读取确保了CORE_DIR一定存在
    // 下面只看配置文件是否存在
    if !path.exists() {
        let config = ArchiverConfig::default();
        // 不能使用config::save，因为此函数会用到CONFIG_FILE_PATH导致循环引用
        let json_str = uoe_result!(config.to_formatted_string(), "");
        uoe_result!(fs::write(&path, json_str), "");
        return path;
    }

    if path.is_dir() {
        panic!(
            "'{}' should be a json file, but got a directory",
            path.force_to_string()
        );
    }

    path
});

/// 自增主键文件路径
/// - 该文件存放在CORE_DIR下
/// - 如果文件不存在，则创建一个默认的
/// - 如果是目录，则panic
pub static AUTO_INCR_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = ROOT_DIR.join(consts::CORE_DIR).join(consts::AUTO_INCR_FILE);
    if !path.exists() {
        let json = uoe_result!(
            AutoIncr::default().to_formatted_string(),
            "Failed to serialize AutoIncr"
        );
        uoe_result!(
            fs::write(&path, json),
            "Failed to create auto increment file"
        );
    }

    if path.is_dir() {
        panic!(
            "'{}' should be a json file, but got a directory",
            path.force_to_string()
        );
    }

    path
});

/// 归档记录文件路径
pub static LIST_FILE_PATH: Lazy<PathBuf> =
    Lazy::new(|| ROOT_DIR.join(consts::CORE_DIR).join(consts::LIST_FILE));

/// 库列表文件路径
pub static VAULTS_FILE_PATH: Lazy<PathBuf> =
    Lazy::new(|| ROOT_DIR.join(consts::CORE_DIR).join(consts::VAULTS_FILE));

/// 别名映射表
/// - 专门给下面的apply_alias函数使用
/// - 该表存放在配置文件中
static ALIAS_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let content = uoe_result!(
        fs::read_to_string(CONFIG_FILE_PATH.as_path()),
        "Cannot read config file"
    );

    let config = uoe_result!(
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

// # 与路径相关的函数

/// 根据归档id和vault_id获取归档对象的路径
pub fn get_archived_path(archive_id: u32, vault_id: u32) -> PathBuf {
    let path = VAULTS_DIR
        .join(vault_id.to_string())
        .join(archive_id.to_string());
    path
}

/// 将alias应用到一串路径字符串里
pub fn apply_alias(path_str: &str) -> String {
    use std::path::MAIN_SEPARATOR;
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
                    // 找到单词边界
                    let mut left = i;
                    let mut right = i + olen;
                    // 向左扩展
                    while left > 0
                        && (bytes[left - 1].is_ascii_alphanumeric() || bytes[left - 1] == b'_')
                    {
                        left -= 1;
                    }
                    // 向右扩展
                    while right < len
                        && (bytes[right].is_ascii_alphanumeric() || bytes[right] == b'_')
                    {
                        right += 1;
                    }
                    // 检查单词是否包含 MAIN_SEPARATOR
                    let word = &path_str[left..right];
                    if word.contains(MAIN_SEPARATOR) {
                        let mut replaced = path_str.to_string();
                        replaced.replace_range(i..i + olen, alias);
                        return replaced;
                    }
                }
            }
            i += 1;
        }
    }
    path_str.to_string()
}

/// 根据年份获取日志文件路径
pub fn get_log_path(year: u32) -> PathBuf {
    LOGS_DIR.join(format!("{}.jsonl", year))
}

/// 获取logs文件夹下所有日志的年份，从大到小排列
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

/// 获取所有库的list路径
/// - 该函数会遍历VAULTS_DIR下的所有文件夹
pub fn get_all_list_paths() -> Vec<PathBuf> {
    let entries: fs::ReadDir = uoe_result!(
        fs::read_dir(VAULTS_DIR.as_path()),
        "Failed get_lists_of_all_vaults"
    );

    entries
        .map(|entry| {
            let entry = uoe_result!(entry, "Failed to read entry");
            entry.path()
        })
        .collect()
}

pub fn get_default_vault_path() -> PathBuf {
    let path = VAULTS_DIR.join(consts::DEFAULT_VAULT);
    if !path.exists() {
        uoe_result!(
            fs::create_dir_all(&path),
            "Failed to create DEFAULT_VAULT directory"
        );
    }
    path
}

// todo 这里要添加config的默认vault_id，就能直接获取不需要id了
