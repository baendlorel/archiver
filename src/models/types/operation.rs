use clap::builder::Str;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// todo 感觉这里是重复代码，可能应该impl在archivercommander里面
#[derive(Serialize, Deserialize, Clone)]
pub struct Operation {
    /// 主命令，比如put、restore
    main: String,

    /// 子命令，比如vault的create、use
    sub: String,

    /// 参数，直接跟在主/子命令后面的
    /// - 有了子命令时，主命令不会有参数
    args: String,

    /// 选项，类似于--key=value的形式，不会保存“--”
    opts: HashMap<String, String>,
}

impl Operation {
    pub fn new(main: &str, sub: &str, args: &str, opts: HashMap<String, String>) -> Self {
        Self {
            main: main.to_string(),
            sub: sub.to_string(),
            args: args.to_string(),
            opts,
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = format!("{} {} {}", self.main, self.sub, self.args);
        for (key, value) in &self.opts {
            result.push_str(&format!(" --{}={}", key, value));
        }
        result
    }

    pub fn to_colored_string(&self) -> String {
        let mut result = format!(
            "{} {} {}",
            self.main.bold().green(),
            self.sub.bold().blue(),
            self.args.bold().yellow()
        );
        for (key, value) in &self.opts {
            result.push_str(&format!(
                " --{}={}",
                key.bold().cyan(),
                value.bold().magenta()
            ));
        }
        result
    }
}
