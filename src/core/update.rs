use crate::{info, must_ok};

use std::fs;
use std::os::unix::process::CommandExt;
use std::{cmp::Ordering, process::Command}; // for exec

use super::config::{CONFIG, update_check};
use crate::misc::{mark, paths};
use crate::models::{error::ArchiverResult, types::Version};
use crate::traits::{ForceToString, ResultExt};

const GITHUB_API_URL: &str = "https://api.github.com/repos/baendlorel/archiver/releases/latest";
const SCRIPT_URL: &str =
    "https://github.com/baendlorel/archiver/releases/download/scripts/archiver-installer.sh";

/// 和上面的区别在于版本相同时静默
pub fn auto_check() {
    // 只在config为真时进行
    if CONFIG.update_check == "off" {
        return;
    }

    // 一个月看一次
    if !update_check::time_passed(&CONFIG) {
        return;
    }

    // 获取当前版本
    let (current, latest) = match prepare_versions() {
        Ok(v) => v,
        Err(e) => {
            e.display();
            return;
        }
    };

    match latest.cmp(&current) {
        Ordering::Greater => println!(
            "{} New version available! Please run `arv update`",
            mark::warn()
        ),
        Ordering::Less => println!("{} How could you use a newer version?", mark::warn()),
        Ordering::Equal => {} // 自动检测的话，版本相同旧无所谓
    }

    // 检查过了更新，刷新一下检测记录
    update_check::refresh_last_date().allow_and_display();
}

pub fn prepare_versions() -> ArchiverResult<(Version, Version)> {
    let current = Version::from(env!("CARGO_PKG_VERSION"));
    // 通过 GitHub API 获取最新 release
    let output = Command::new("curl")
        .arg("-s")
        .arg(GITHUB_API_URL)
        .arg("-H")
        .arg("User-Agent: archiver-cli") // GitHub API 需要 User-Agent
        .output();

    let output = match output {
        Ok(o) => o,
        Err(e) => {
            return info!("curl failed: {}", e);
        }
    };

    let json = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(e) => {
            return info!("output decode failed: {}", e.to_string());
        }
    };

    // 解析 tag_name 字段
    let latest_version = json
        .split("\"tag_name\":\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .unwrap_or("");

    if latest_version.is_empty() {
        return info!(
            "Failed to parse latest version from GitHub releases. Response: {}",
            json
        );
    }

    let latest = Version::from(latest_version);
    Ok((current, latest))
}

pub fn reinstall() {
    // 1. 下载脚本
    let script_path = paths::ROOT_DIR.join("archiver-installer.sh");

    if script_path.exists() {
        must_ok!(fs::remove_file(&script_path), "Fail to remove old script");
        println!(
            "{} old script: '{}' removed",
            mark::succ(),
            script_path.force_to_string()
        );
    }

    println!(
        "{} Downloading script from '{}' to '{}'",
        mark::info(),
        SCRIPT_URL,
        script_path.force_to_string()
    );

    let status = std::process::Command::new("curl")
        .arg("-fsSL")
        .arg("-o")
        .arg(script_path.force_to_string())
        .arg(SCRIPT_URL)
        .status();

    match status {
        Ok(s) => {
            if s.success() {
                println!("{} script downloaded", mark::succ());
            } else {
                eprintln!("{} Failed to download update script: {}", mark::error(), s);
                return;
            }
        }
        Err(e) => {
            eprintln!("{} Failed to download update script: {}", mark::error(), e);
            return;
        }
    }

    // 2. 设置可执行权限
    let status = std::process::Command::new("chmod")
        .arg("+x")
        .arg(script_path.force_to_string())
        .status();

    match status {
        Ok(s) => {
            if s.success() {
                println!("{} script chmoded", mark::succ());
            } else {
                eprintln!("{} Failed to chmod update script: {}", mark::error(), s);
                return;
            }
        }
        Err(e) => {
            eprintln!("{} Failed to chmod update script: {}", mark::error(), e);
            return;
        }
    }

    println!("{} script is ready, executing...", mark::succ());

    // 3. 用 exec 替换当前进程
    let err = std::process::Command::new("sh")
        .arg(script_path.force_to_string())
        .exec();

    // exec 只会在出错时返回
    eprintln!("{} Failed to exec update script: {}", mark::error(), err);
    std::process::exit(1);
}
