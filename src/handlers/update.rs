use owo_colors::OwoColorize;
use std::process::Command;

/// 检查是否有新版本可用（从 GitHub Releases 获取）
pub fn handler() {
    // 获取当前版本
    let current_version = env!("CARGO_PKG_VERSION");
    println!("Current version: {}", current_version.cyan());

    // 通过 GitHub API 获取最新 release
    let output = Command::new("curl")
        .arg("-s")
        .arg("https://api.github.com/repos/kasukabe-tsumugi/archiver/releases/latest")
        .arg("-H")
        .arg("User-Agent: archiver-cli") // GitHub API 需要 User-Agent
        .output();

    let output = match output {
        Ok(o) => o,
        Err(e) => {
            println!("{} curl failed: {}", "[update]".yellow(), e);
            return;
        }
    };

    let json = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(e) => {
            println!("{} output decode failed: {}", "[update]".yellow(), e);
            return;
        }
    };

    // 解析 tag_name 字段
    let latest_version = json
        .split("\"tag_name\":\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .unwrap_or("");

    if latest_version.is_empty() {
        println!(
            "{} Failed to parse latest version from GitHub releases",
            "[update]".yellow()
        );
        return;
    }

    println!("Latest release: {}", latest_version.green());
    // 去掉 tag 前缀 v
    let latest_version_trimmed = latest_version.trim_start_matches('v');
    if latest_version_trimmed > current_version {
        println!(
            "{} New version available! Please update.",
            "[update]".green()
        );
    } else {
        println!("{} You are using the latest version.", "[update]".cyan());
    }
}
