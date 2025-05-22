use crate::{
    handlers::log,
    misc::{CONFIG_VALID_STMT, mark},
    models::types::OperType,
};

mod alias;
pub mod auto_check_update;
mod data;
mod display;

pub use data::load;

pub fn handler(statement: &Option<Vec<String>>) {
    // 输入arv config，后面没了，就会进入此分支
    // 直接打印所有配置
    if statement.is_none() {
        handle_show(&None);
        return;
    }

    // arv config 后面有后续，那么args数组至少有1项
    let stmt = statement.as_ref().unwrap();

    let show_standard_form = |problem: &str| {
        let head = format!(
            "Invalid config statement: {}, got `{}`",
            problem,
            stmt.join(" ")
        );
        println!("{} {}", mark::fail(), head);
        println!("{}", CONFIG_VALID_STMT.as_str());
    };

    // 内部写一个handler，为了避免每次运行结束都要手写一遍show_standard_form
    let _handler = || -> &str {
        // 配置语句只有一个词，那么可能是配置项<item>或者配置项+指令<item>.<dirv>
        // arv config xxx 或arv config xxx.action
        // 根据`.`来split
        if stmt.len() == 1 {
            let item_dirv = stmt[0].split(".").collect::<Vec<&str>>();
            // split后只有一项，说明输入的是配置项名字（或其他东西，将会在更里层报错）
            if item_dirv.len() == 1 {
                handle_show(&Some(stmt[0].to_string()));
                return "";
            }

            // split后不止一项，但现有的配置语句一定是要参数的，所以不成立，返回
            return "got no value";
        }

        // * 进入配置语句解析环节
        // 配置语句（目前）必须是2个词的，第一个词是<item>.<dirv>，第二个是<value>
        if stmt.len() != 2 {
            return "statement must contain 2 words exactly";
        }

        // 解析<item>.<dirv>
        let item_dirv = stmt[0].split_once(".");
        if item_dirv.is_none() {
            // 只输入了配置项的名字，打印所有配置项
            return "missing directive";
        }

        let item_dirv = item_dirv.unwrap();

        let item = item_dirv.0;
        let dirv = item_dirv.1;
        let arg = &stmt[1];

        match item {
            "alias" => match dirv {
                "add" => handle_add_alias(arg),
                "remove" => handle_remove_alias(arg),
                _ => return "invalid directive, must be `add`/`remove`",
            },
            "auto-check-update" => match dirv {
                "set" => handle_auto_check_update(arg),
                _ => return "invalid directive, must be `set`",
            },
            _ => return "unsupported config item",
        };

        return "";
    };

    let problem = _handler();
    if !problem.is_empty() {
        show_standard_form(problem);
    }
}

fn handle_show(config_item: &Option<String>) {
    if let Err(e) = display::display(config_item) {
        e.display();
    }
}

fn handle_add_alias(arg: &str) {
    let oper = OperType::Config {
        option: "alias.add".to_string(),
    };
    match alias::set_alias(&arg) {
        Ok(_) => {
            let msg = format!("Alias '{}' is set successfully.", arg);
            log::succ(&oper, arg, None, &msg);
        }
        Err(e) => log::err(&oper, arg, None, e),
    }
}

fn handle_remove_alias(arg: &str) {
    let oper = OperType::Config {
        option: "alias.remove".to_string(),
    };

    match alias::remove_alias(&arg) {
        Ok(_) => {
            let msg = format!("Alias '{}' is removed successfully.", arg);
            log::succ(&oper, arg, None, &msg);
        }
        Err(e) => log::err(&oper, arg, None, e),
    }
}

fn handle_auto_check_update(arg: &str) {
    let oper = OperType::Config {
        option: "auto-check-update".to_string(),
    };

    match auto_check_update::toggle(&arg) {
        Ok(_) => {
            let msg = format!("Auto check update is set to '{}'.", arg);
            log::succ(&oper, arg, None, &msg);
        }
        Err(e) => log::err(&oper, arg, None, e),
    }
}
