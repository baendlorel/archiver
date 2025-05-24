use std::process::exit;

use crate::{
    info,
    misc::{CONFIG_VALID_STMT, mark},
    models::error::ArchiverError,
};

pub enum ConfigCommand {
    Display {
        item: Option<String>,
    },
    Alias {
        add: Option<String>,
        remove: Option<String>,
    },
    AutoCheckUpdate {
        set: String,
    },
}

pub fn parse_command(statement: &Option<Vec<String>>) -> ConfigCommand {
    // 输入arv config，后面没了，就会进入此分支
    // 直接打印所有配置
    if statement.is_none() {
        // display(&None);
        return ConfigCommand::Display { item: None };
    }

    // arv config 后面有后续，那么args数组至少有1项
    let stmt = statement.as_ref().unwrap();

    // 内部写一个handler，为了避免每次运行结束都要手写一遍show_standard_form
    let _handler = || -> Result<ConfigCommand, ArchiverError> {
        // 配置语句只有一个词，那么可能是配置项<item>或者配置项+指令<item>.<dirv>
        // arv config xxx 或arv config xxx.action
        // 根据`.`来split
        if stmt.len() == 1 {
            let item_dirv = stmt[0].split(".").collect::<Vec<&str>>();
            // split后只有一项，说明输入的是配置项名字（或其他东西，将会在更里层报错）
            if item_dirv.len() == 1 {
                // config::display(&Some(stmt[0].to_string()));
                // return "";
                return Ok(ConfigCommand::Display {
                    item: Some(stmt[0].to_string()),
                });
            }

            // split后不止一项，但现有的配置语句一定是要参数的，所以不成立，返回
            return info!("got no value");
        }

        // * 进入配置语句解析环节
        // 配置语句（目前）必须是2个词的，第一个词是<item>.<dirv>，第二个是<value>
        if stmt.len() != 2 {
            return info!("statement must contain 2 words exactly");
        }

        // 解析<item>.<dirv>
        let item_dirv = stmt[0].split_once(".");
        if item_dirv.is_none() {
            // 只输入了配置项的名字，打印所有配置项
            return info!("missing directive");
        }

        let item_dirv = item_dirv.unwrap();

        let item = item_dirv.0;
        let dirv = item_dirv.1;
        let arg = &stmt[1];

        match item {
            "alias" => match dirv {
                "add" => Ok(ConfigCommand::Alias {
                    add: Some(arg.to_string()),
                    remove: None,
                }), // config::add_alias(arg),
                "remove" => Ok(ConfigCommand::Alias {
                    add: None,
                    remove: Some(arg.to_string()),
                }), // config::remove_alias(arg),
                _ => return info!("invalid directive, must be `add`/`remove`"),
            },
            "auto-check-update" => match dirv {
                "set" => Ok(ConfigCommand::AutoCheckUpdate {
                    set: arg.to_string(),
                }), // config::auto_check_update(arg),
                _ => return info!("invalid directive, must be `set`"),
            },
            _ => return info!("unsupported config item"),
        }
    };

    match _handler() {
        Ok(command) => command,
        Err(e) => {
            let head = format!(
                "Invalid config statement: {}, got `{}`",
                e.to_string(),
                stmt.join(" ")
            );
            println!("{} {}", mark::fail(), head);
            println!("{}", CONFIG_VALID_STMT.as_str());
            exit(1);
        }
    }
}
