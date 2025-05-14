use std::{fs, io::Write};

use crate::{
    handlers::log,
    misc::paths,
    models::{errors::ConfigError, types::OperType},
};

pub fn handler_alias(alias: Option<String>) {
    if alias.is_none() {
        return;
    }

    let alias_config = alias.unwrap();

    // TODO 要判定是否有重复的配置，重复就不配置了

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

pub fn handler_show_alias(show_alias: bool) {
    if !show_alias {
        return;
    }

    match fs::read_to_string(paths::DIR_ALIAS_FILE_PATH.clone()) {
        Ok(content) => println!(
            "~={}\n{}",
            paths::HOME_DIR.to_string_lossy().to_string(),
            content
        ),
        Err(e) => println!("Show aliases failed. {}", e.to_string()),
    }
}

fn set_alias(alias_config: &String) -> Result<(), ConfigError> {
    // 格式必须是 xxx=/sdf/sdf 的样子
    if let Some((alias, origin)) = alias_config.split_once("=") {
        // paths::DIR_ALIAS_FILE_PATH;
        if alias.is_empty() {
            return Err(ConfigError::EmptyName(format!(
                "alias is empty. Got {}",
                alias_config
            )));
        }

        if origin.is_empty() {
            return Err(ConfigError::EmptyName(format!(
                "origin is empty. Got {}",
                alias_config
            )));
        }

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(paths::DIR_ALIAS_FILE_PATH.clone())?;

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
