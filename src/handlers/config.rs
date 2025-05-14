use std::{fs, io::Write, path};

use owo_colors::OwoColorize;

use crate::{
    handlers::log,
    misc::paths,
    models::{errors::ConfigError, types::OperType},
};

pub fn handler_alias(arg: String) {
    let oper = OperType::Config {
        option: "--alias".to_string(),
    };
    match set_alias(&arg) {
        Ok(_) => {
            println!("Alias '{}' is set successfully.", arg);
            let _ = log::save(oper, arg, true, None, None);
        }
        Err(e) => {
            println!("{}", e.to_string());
            let _ = log::save(oper, arg, false, None, Some(e.to_string()));
        }
    }
}

pub fn handler_alias_list() {
    match fs::read_to_string(paths::DIR_ALIAS_FILE_PATH.clone()) {
        Ok(content) => {
            let home_alias = format!(
                "~={} {}",
                paths::HOME_DIR.to_string_lossy().to_string(),
                "(default)".cyan()
            );
            println!("{}\n{}", home_alias, content)
        }
        Err(e) => println!("Show aliases failed. {}", e.to_string()),
    }
}

pub fn handler_alias_remove(arg: String) {
    let oper = OperType::Config {
        option: "--alias-remove".to_string(),
    };
    match alias_remove(&arg) {
        Ok(_) => {
            println!("Alias '{}' is removed successfully.", arg);
            let _ = log::save(oper, arg, true, None, None);
        }
        Err(e) => {
            println!("{}", e.to_string());
            let _ = log::save(oper, arg, false, None, Some(e.to_string()));
        }
    }
}
fn alias_remove(alias_entry: &String) -> Result<(), ConfigError> {
    let file_path = paths::DIR_ALIAS_FILE_PATH.clone();
    let content = fs::read_to_string(&file_path)?;

    let mut removed_content = String::from("");
    let mut found = false;
    for line in content.lines() {
        if line == alias_entry {
            found = true;
        } else {
            removed_content.push_str(format!("{}\n", line).as_str());
        }
    }
    if !found {
        return Err(ConfigError::AliasNotFound(format!(
            "There is no alias entry = '{}'",
            alias_entry
        )));
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(false)
        .open(&file_path)?;

    // 写入
    file.write_all(removed_content.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}

fn set_alias(alias_entry: &String) -> Result<(), ConfigError> {
    // 格式必须是 xxx=/sdf/sdf 的样子
    if let Some((alias, origin)) = alias_entry.split_once("=") {
        if alias.is_empty() {
            return Err(ConfigError::EmptyName(format!(
                "alias is empty. Got '{}'",
                alias_entry
            )));
        }

        if origin.is_empty() {
            return Err(ConfigError::EmptyName(format!(
                "origin is empty. Got '{}'",
                alias_entry
            )));
        }

        // 去掉origin后面的斜杠
        let origin = origin.trim_end_matches(path::MAIN_SEPARATOR);

        if origin == paths::HOME_DIR.to_string_lossy() {
            return Err(ConfigError::AliasAlreadyExists(
                "HOME_DIR already has a default alias '~', no need to set it again.".to_string(),
            ));
        }

        let file_path = paths::DIR_ALIAS_FILE_PATH.clone();
        let content = fs::read_to_string(&file_path)?;
        for line in content.lines() {
            if line == alias_entry {
                return Err(ConfigError::AliasAlreadyExists(format!(
                    "Got '{}'",
                    alias_entry
                )));
            }
        }

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)?;

        // 写入
        file.write_all(alias_entry.as_bytes())?;
        file.write_all(b"\n")?;

        return Ok(());
    }

    return Err(ConfigError::InvalidAliasEntryForm(format!(
        "Alias config string must take the form of 'xxx=/a/b'. Got '{}'",
        alias_entry
    )));
}
