use crate::{err, wrap_err};

use owo_colors::OwoColorize;
use std::{fs, path};

use crate::misc::ForceToString;
use crate::models::types::{AliasEntry, ArchiverConfig};
use crate::{
    handlers::log,
    misc::paths,
    models::{error::ArchiverError, types::OperType},
};

// # handlers
pub fn handler_alias(arg: String) {
    let oper = OperType::Config {
        option: "--alias".to_string(),
    };
    match set_alias(&arg) {
        Ok(_) => {
            println!("Alias '{}' is set successfully.", arg);
            log::succ(oper, arg, None, None);
        }
        Err(e) => log::err(oper, arg, None, e),
    }
}

pub fn handler_alias_list() {
    match load() {
        Ok(configs) => {
            println!(
                "Alias entries:\n  ~={} {}",
                paths::HOME_DIR.to_string_lossy().to_string(),
                "(default)".cyan()
            );
            for entry in configs.alias_list {
                let content = format!("{}={}", entry.alias, entry.origin,);
                println!("  {}", content);
            }
        }
        Err(e) => println!("Show aliases failed. {}", e.to_string()),
    }
}

pub fn handler_alias_remove(arg: String) {
    let oper = OperType::Config {
        option: "--alias-remove".to_string(),
    };
    match remove_alias(&arg) {
        Ok(_) => {
            println!("Alias '{}' is removed successfully.", arg);
            log::succ(oper, arg, None, None);
        }
        Err(e) => log::err(oper, arg, None, e),
    }
}

// # 业务函数
fn remove_alias(alias_entry: &String) -> Result<(), ArchiverError> {
    let (alias, origin) = wrap_err!(parse_alias_entry_string(alias_entry))?;
    let mut configs = load()?;

    let mut index = 0;
    for entry in &configs.alias_list {
        if entry.alias == alias && entry.origin == origin {
            break;
        }
        index += 1;
    }

    configs.alias_list.splice(index..index, []);
    wrap_err!(save(&configs))?;

    Ok(())
}

fn set_alias(alias_entry: &String) -> Result<(), ArchiverError> {
    let (alias, origin) = wrap_err!(parse_alias_entry_string(alias_entry))?;
    let mut configs = wrap_err!(load())?;

    for entry in &configs.alias_list {
        if entry.alias == alias {
            return Err(err!(format!(
                "Alias '{}' is already bound with origin '{}'",
                entry.alias, entry.origin
            )));
        }
        if entry.origin == origin {
            return Err(err!(format!(
                "Origin '{}' is already bound with alias '{}'",
                entry.origin, entry.alias
            )));
        }
    }

    configs.alias_list.push(AliasEntry {
        alias: alias.to_string(),
        origin: origin.to_string(),
    });
    wrap_err!(save(&configs))?;

    Ok(())
}

// # 辅助函数
fn parse_alias_entry_string(alias_entry: &String) -> Result<(String, String), ArchiverError> {
    if let Some((alias, origin)) = alias_entry.split_once("=") {
        if alias.is_empty() {
            return Err(err!(format!("alias is empty. Got '{}'", alias_entry)));
        }

        if origin.is_empty() {
            return Err(err!(format!("origin is empty. Got '{}'", alias_entry)));
        }

        // 去掉origin后面的斜杠
        let alias = alias.trim_end_matches(path::MAIN_SEPARATOR);
        let origin = origin.trim_end_matches(path::MAIN_SEPARATOR);

        if alias == origin {
            return Err(err!(format!(
                "Alias and origin cannot be the same. Got '{}'",
                alias_entry
            )));
        }

        if origin == paths::HOME_DIR.force_to_string() || alias == "~" {
            return Err(err!(
                "HOME_DIR already has a default alias '~', no need to set it again."
            ));
        }
        return Ok((alias.to_string(), origin.to_string()));
    } else {
        Err(err!(format!(
            "Alias config string must take the form of 'xxx=/a/b'. Got '{}'",
            alias_entry
        )))
    }
}

fn load() -> Result<ArchiverConfig, ArchiverError> {
    let content = wrap_err!(fs::read_to_string(paths::CONFIG_FILE_PATH.clone()))?;
    Ok(wrap_err!(serde_json::from_str::<ArchiverConfig>(&content))?)
}

fn save(configs: &ArchiverConfig) -> Result<(), ArchiverError> {
    let json_str = wrap_err!(serde_json::to_string_pretty(configs))?;
    wrap_err!(fs::write(paths::CONFIG_FILE_PATH.clone(), json_str))?;
    Ok(())
}
