use std::fs;
use std::path::PathBuf;

const ROOT: &str = ".archiver";
const LOGS_DIR: &str = "logs";
const LIST_FILE: &str = "archive-list.jsonl";
const AUTO_INCR_FILE: &str = "auto-incr";

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
    let path = root_dir().join(LOGS_DIR);
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create logs_dir directory");
    }
    path
}

pub fn list_file_path() -> PathBuf {
    let path = root_dir();
    path.join(LIST_FILE)
}

pub fn auto_incr_id() -> u32 {
    let auto_incr_file = root_dir().join(AUTO_INCR_FILE);
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
