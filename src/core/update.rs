use crate::{err_info, log_if_err, uoe_result};

use std::fs;
use std::os::unix::process::CommandExt;
use std::{cmp::Ordering, process::Command}; // for exec

use crate::{
    misc::{ForceToString, mark, paths},
    models::{error::ArchiverError, types::Version},
};

use super::config::{auto_check_update, load};

const GITHUB_API_URL: &str = "https://api.github.com/repos/baendlorel/archiver/releases/latest";
const SCRIPT_URL: &str =
    "https://github.com/baendlorel/archiver/releases/download/scripts/archiver-installer.sh";

/// 和上面的区别在于版本相同时静默
pub fn auto_check() {
    let mut config = match load() {
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

    // 一个月看一次
    if !auto_check_update::time_passed(&config) {
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
    log_if_err!(auto_check_update::refresh(&mut config));
}

pub fn prepare_versions() -> Result<(Version, Version), ArchiverError> {
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
    Ok((current, latest))
}

pub fn reinstall() {
    // 1. 下载脚本
    let script_path = paths::ROOT_DIR.join("archiver-installer.sh");

    if script_path.exists() {
        uoe_result!(fs::remove_file(&script_path), "Fail to remove old script");
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
                eprintln!("{} Failed to download update script: {}", mark::fail(), s);
                return;
            }
        }
        Err(e) => {
            eprintln!("{} Failed to download update script: {}", mark::fail(), e);
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
                eprintln!("{} Failed to chmod update script: {}", mark::fail(), s);
                return;
            }
        }
        Err(e) => {
            eprintln!("{} Failed to chmod update script: {}", mark::fail(), e);
            return;
        }
    }

    println!("{} script is ready, executing...", mark::succ());

    // 3. 用 exec 替换当前进程
    let err = std::process::Command::new("sh")
        .arg(script_path.force_to_string())
        .exec();

    // exec 只会在出错时返回
    eprintln!("{} Failed to exec update script: {}", mark::fail(), err);
    std::process::exit(1);
}
