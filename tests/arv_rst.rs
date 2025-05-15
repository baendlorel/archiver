use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_archive_and_restore() {
    // 假设有可执行文件 archiver 或用 cargo run
    let test_file = "test_file.txt";
    fs::write(test_file, "hello").unwrap();

    // 归档
    let output = Command::new("cargo")
        .args(["run", "--", "put", test_file])
        .output()
        .expect("failed to execute process");
    assert!(output.status.success());
    assert!(!Path::new(test_file).exists());
    let id = {
        let s = String::from_utf8_lossy(&output.stdout);
        if let Some(pos) = s.rfind(':') {
            let tail = s[pos + 1..].trim();
            tail.to_string()
        } else {
            panic!("不应该");
        }
    };

    // 恢复
    let output = Command::new("cargo")
        .args(["run", "--", "restore", &id])
        .output()
        .expect("failed to execute process");
    assert!(output.status.success());
    assert!(Path::new(test_file).exists());

    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_alias_config() {
    let arg = "testalias=/tmp";
    // 添加别名
    let output = Command::new("cargo")
        .args(["run", "--", "config", "--alias", arg])
        .output()
        .expect("failed to execute process");
    assert!(output.status.success());

    // 查询别名
    let output = Command::new("cargo")
        .args(["run", "--", "config", "--alias-list"])
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(arg));

    let output = Command::new("cargo")
        .args(["run", "--", "config", "--alias-remove", arg])
        .output()
        .expect("failed to execute process");
    assert!(output.status.success());

    // 查询别名
    let output = Command::new("cargo")
        .args(["run", "--", "config", "--alias-list"])
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains(arg));
}
