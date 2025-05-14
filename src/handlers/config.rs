use std::{fs, io::Write, path};

use owo_colors::OwoColorize;

use crate::{
    handlers::log,
    misc::paths,
    models::{errors::ConfigError, types::OperType},
};

// TODO 增加删除alias的功能
pub fn handler_alias(alias: Option<String>) {
    if alias.is_none() {
        return;
    }

    let alias_config = alias.unwrap();

    match set_alias(&alias_config) {
        Ok(_) => {
            println!("Alias '{}' is set successfully.", alias_config);
            let _ = log::save(OperType::Config, alias_config, true, None, None);
        }
        Err(e) => {
            println!("{}", e.to_string());
            let _ = log::save(
                OperType::Config,
                alias_config,
                false,
                None,
                Some(e.to_string()),
            );
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

fn set_alias(alias_config: &String) -> Result<(), ConfigError> {
    // 格式必须是 xxx=/sdf/sdf 的样子
    if let Some((alias, origin)) = alias_config.split_once("=") {
        if alias.is_empty() {
            return Err(ConfigError::EmptyName(format!(
                "alias is empty. Got '{}'",
                alias_config
            )));
        }

        if origin.is_empty() {
            return Err(ConfigError::EmptyName(format!(
                "origin is empty. Got '{}'",
                alias_config
            )));
        }

        // 去掉origin后面的斜杠
        let origin = origin.trim_end_matches(path::MAIN_SEPARATOR);

        if origin == paths::HOME_DIR.to_string_lossy() {
            return Err(ConfigError::AliasAlreadyExists(
                "HOME_DIR has a default alias '~', there is no need to set it again.".to_string(),
            ));
        }

        let file_path = paths::DIR_ALIAS_FILE_PATH.clone();

        let content = fs::read_to_string(&file_path)?;

        for line in content.lines() {
            if line == alias_config {
                return Err(ConfigError::AliasAlreadyExists(format!(
                    "Got '{}'",
                    alias_config
                )));
            }
        }

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)?;

        // 写入
        file.write_all(alias_config.as_bytes())?;
        file.write_all(b"\n")?;

        return Ok(());
    }

    return Err(ConfigError::InvalidAliasEntryForm(format!(
        "Alias config string must take the form of 'xxx=/a/b'. Got '{}'",
        alias_config
    )));
}
