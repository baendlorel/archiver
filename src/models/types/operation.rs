use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Operation {
    // todo 这里是不是写成enum比较好?
    /// 主命令，比如put、restore
    pub main: String,

    /// 子命令，比如vault的create、use
    pub sub: String,

    /// 参数，直接跟在主/子命令后面的
    /// - 有了子命令时，主命令不会有参数
    pub args: Vec<String>,

    /// 选项，类似于--key=value的形式，不会保存“--”
    pub opts: HashMap<String, Value>,
}

impl Operation {
    pub fn simple(main: &str, args: Vec<String>, opts: HashMap<String, Value>) -> Self {
        Self {
            main: main.to_string(),
            sub: String::new(),
            args: args,
            opts,
        }
    }

    pub fn new(main: &str, sub: &str, args: Vec<String>, opts: HashMap<String, Value>) -> Self {
        Self {
            main: main.to_string(),
            sub: sub.to_string(),
            args: args,
            opts,
        }
    }

    pub fn to_string(&self) -> String {
        let mut result: Vec<String> = vec![self.main.clone()];
        if !self.sub.is_empty() {
            result.push(self.sub.clone());
        }

        if !self.args.is_empty() {
            result.append(self.args.clone().as_mut());
        }

        for (key, value) in &self.opts {
            result.push(format!("--{}={}", key, value));
        }

        result.join(" ")
    }

    pub fn to_colored_string(&self) -> String {
        let mut result = format!(
            "{} {} {}",
            self.main.bold().green(),
            self.sub.bold().blue(),
            self.args.join(" ").bold().yellow()
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
