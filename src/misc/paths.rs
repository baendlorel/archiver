use std::fs;
use std::path::PathBuf;

const ROOT: &str = ".archiver";
const OPER_LOG: &str = "operation-log";
const LIST_FILE: &str = "archive-list.jsonl";

fn assure_root() -> PathBuf {
    let mut path = dirs::home_dir().expect("Failed to get home directory");
    path.push(ROOT);
    // 检查路径是否存在，不存在则创建
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create root directory");
    }
    path
}

pub fn logs_dir() -> PathBuf {
    let mut path = assure_root();
    path.push(OPER_LOG);
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create logs_dir directory");
    }
    path
}

pub fn list_file_path() -> PathBuf {
    let mut path = assure_root();
    path.push(LIST_FILE);
    path
}
