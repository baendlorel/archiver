use crate::must_ok;

use once_cell::sync::Lazy;
use std::fs;
use std::path::PathBuf;

use crate::models::{
    serde_custom::SerdeJson,
    types::{ArchiverConfig, AutoIncrVars},
};
use crate::traits::ForceToString;

mod raw {
    // 目录
    pub const ROOT: &str = ".archiver";
    pub const LOGS_DIR: &str = "logs";
    pub const CORE_DIR: &str = "core";
    pub const VAULTS_DIR: &str = "vaults";

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
                $crate::must_ok!(
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
    Lazy::new(|| must_ok!(std::env::current_dir(), "Failed to get current directory"));

/// 用户文件夹
#[cfg(not(feature = "dev"))]
pub static HOME_DIR: Lazy<PathBuf> =
    Lazy::new(|| crate::must_some!(dirs::home_dir(), "Failed to get home directory"));

/// 当前工作目录
pub static CWD: Lazy<PathBuf> =
    Lazy::new(|| must_ok!(std::env::current_dir(), "Failed to get current directory"));

/// 程序主目录
pub static ROOT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = HOME_DIR.join(raw::ROOT);
    // 检查路径是否存在，不存在则创建
    if !path.exists() {
        must_ok!(fs::create_dir_all(&path), "Failed to create root directory");
    }
    path
});

/// 日志目录
pub static LOGS_DIR: Lazy<PathBuf> = ensure_dir!(ROOT_DIR.join(raw::LOGS_DIR));

/// 配置目录
pub static CORE_DIR: Lazy<PathBuf> = ensure_dir!(ROOT_DIR.join(raw::CORE_DIR));

/// 归档的文件/文件夹存放的地方
pub static VAULTS_DIR: Lazy<PathBuf> = ensure_dir!(ROOT_DIR.join(raw::VAULTS_DIR));

/// 配置文件路径
/// - 该文件存放在CORE_DIR下
/// - 如果文件不存在，则创建一个默认的配置文件。因为配置文件总要读取，必须存在
/// - 如果是目录，则panic
pub static CONFIG_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = CORE_DIR.join(raw::CONFIG_FILE);
    // 从CORE_DIR读取确保了CORE_DIR一定存在
    // 下面只看配置文件是否存在
    if !path.exists() {
        let config = ArchiverConfig::default();
        // 不能使用config::save，因为此函数会用到CONFIG_FILE_PATH导致循环引用
        let json_str = must_ok!(config.to_formatted_string(), "");
        must_ok!(fs::write(&path, json_str), "");
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
    let path = ROOT_DIR.join(raw::CORE_DIR).join(raw::AUTO_INCR_FILE);
    if !path.exists() {
        let json = must_ok!(
            AutoIncrVars::default().to_formatted_string(),
            "Failed to serialize AutoIncr"
        );
        must_ok!(
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
    Lazy::new(|| ROOT_DIR.join(raw::CORE_DIR).join(raw::LIST_FILE));

/// 库列表文件路径
pub static VAULTS_FILE_PATH: Lazy<PathBuf> =
    Lazy::new(|| ROOT_DIR.join(raw::CORE_DIR).join(raw::VAULTS_FILE));

// # 与路径相关的函数

/// 根据年份获取日志文件路径
pub fn get_log_path(year: i32) -> PathBuf {
    LOGS_DIR.join(format!("{}.jsonl", year))
}

/// 获取logs文件夹下所有日志的年份，从大到小排列
pub fn get_years_desc() -> Vec<i32> {
    let mut years = Vec::new();
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
                    years.push(year);
                }
            }
        }
    }
    years.sort_by(|a, b| b.cmp(a));
    years
}

pub fn get_vault_path(vault_id: u32) -> PathBuf {
    let path = VAULTS_DIR.join(vault_id.to_string());
    if !path.exists() {
        must_ok!(
            fs::create_dir_all(&path),
            format!("Failed to create VAULTS_DIR for vid:{} directory", vault_id)
        );
    }
    path
}

/// 根据archive_id和vault_id获取归档对象的路径
pub fn get_archived_path(archive_id: u32, vault_id: u32) -> PathBuf {
    let path = VAULTS_DIR
        .join(vault_id.to_string())
        .join(archive_id.to_string());
    path
}
