use std::{fs, io::Write, path};

use owo_colors::OwoColorize;

use crate::{err, wrap_err};
use crate::{
    handlers::log,
    misc::paths,
    models::{error::ArchiverError, types::OperType},
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
fn alias_remove(alias_entry: &String) -> Result<(), ArchiverError> {
    let file_path = paths::DIR_ALIAS_FILE_PATH.clone();
    let content = wrap_err!(fs::read_to_string(&file_path))?;

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
        return Err(err!(format!("There is no alias entry = '{}'", alias_entry)));
    }

    wrap_err!(fs::write(&file_path, removed_content + "\n"))?;
    Ok(())
}

fn set_alias(alias_entry: &String) -> Result<(), ArchiverError> {
    // 格式必须是 xxx=/sdf/sdf 的样子
    if let Some((alias, origin)) = alias_entry.split_once("=") {
        if alias.is_empty() {
            return Err(err!(format!("alias is empty. Got '{}'", alias_entry)));
        }

        if origin.is_empty() {
            return Err(err!(format!("origin is empty. Got '{}'", alias_entry)));
        }

        // 去掉origin后面的斜杠
        let origin = origin.trim_end_matches(path::MAIN_SEPARATOR);

        if origin == paths::HOME_DIR.to_string_lossy() {
            return Err(err!(
                "HOME_DIR already has a default alias '~', no need to set it again."
            ));
        }

        let file_path = paths::DIR_ALIAS_FILE_PATH.clone();
        let content = wrap_err!(fs::read_to_string(&file_path))?;
        for line in content.lines() {
            if line == alias_entry {
                return Err(err!(format!("Got '{}'", alias_entry)));
            }
        }

        let mut file = wrap_err!(
            fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file_path)
        )?;

        // 写入
        wrap_err!(file.write_all(alias_entry.as_bytes()))?;
        wrap_err!(file.write_all(b"\n"))?;

        return Ok(());
    }

    return Err(err!(format!(
        "Alias config string must take the form of 'xxx=/a/b'. Got '{}'",
        alias_entry
    )));
}
