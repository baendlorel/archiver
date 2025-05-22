use crate::err_info;

use owo_colors::OwoColorize;
use std::{cmp::Ordering, process::Command};

use crate::{
    misc::status_mark,
    models::{error::ArchiverError, types::Version},
};

use super::config;

/// 检查是否有新版本可用（从 GitHub Releases 获取）
pub fn handler() {
    // 获取当前版本
    let (cur, latest) = match prepare_versions() {
        Ok(v) => v,
        Err(e) => {
            e.display();
            return;
        }
    };

    // todo 真正实现update
    match latest.cmp(&cur) {
        Ordering::Greater => println!("status_mark::warn() New version available! Please update.",),
        Ordering::Equal => println!("{} You are using the latest version.", status_mark::succ()),
        Ordering::Less => println!("status_mark::warn() How could you use a newer version?"),
    }
}

/// 和上面的区别在于版本相同时静默
pub fn auto_check_update() {
    let config = match config::config_data::load() {
        Ok(c) => c,
        Err(e) => {
            e.display();
            return;
        }
    };

    // 只在config为真时进行
    if config.auto_check_update == "off" {
        return;
    }

    // 获取当前版本
    let (cur, latest) = match prepare_versions() {
        Ok(v) => v,
        Err(e) => {
            e.display();
            return;
        }
    };

    match latest.cmp(&cur) {
        Ordering::Greater => {
            println!("status_mark::warn() New version available! Please download it manually.",)
        }
        Ordering::Less => println!("status_mark::warn() How could you use a newer version?"),
        Ordering::Equal => {}
    }
}

fn prepare_versions() -> Result<(Version, Version), ArchiverError> {
    let current = Version::from(env!("CARGO_PKG_VERSION"));
    // 通过 GitHub API 获取最新 release
    let output = Command::new("curl")
        .arg("-s")
        .arg("https://api.github.com/repos/baendlorel/archiver/releases/latest")
        .arg("-H")
        .arg("User-Agent: archiver-cli") // GitHub API 需要 User-Agent
        .output();

    let output = match output {
        Ok(o) => o,
        Err(e) => {
            return err_info!("curl failed: {}", e);
        }
    };

    let json = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(e) => {
            return err_info!("output decode failed: {}", e.to_string());
        }
    };

    // 解析 tag_name 字段
    let latest_version = json
        .split("\"tag_name\":\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .unwrap_or("");

    if latest_version.is_empty() {
        return err_info!("Failed to parse latest version from GitHub releases");
    }

    let latest = Version::from(latest_version);

    println!("Current version: {}", current.to_string().cyan());
    println!("Latest  release: {}", latest.to_string().green());

    Ok((current, latest))
}
