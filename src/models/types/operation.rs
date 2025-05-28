use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::cli::short;
use crate::traits::CustomColors;

#[derive(Serialize, Deserialize, Clone)]
pub struct Operation {
    /// 主命令，比如put、restore
    #[serde(rename = "m")]
    pub main: String,

    /// 子命令，比如vault的create、use
    #[serde(rename = "s")]
    pub sub: String,

    /// 指令，比如config alias add的add
    #[serde(rename = "d")]
    pub directive: String,

    /// 参数，直接跟在主/子命令后面的
    /// - 有了子命令时，主命令不会有参数
    #[serde(rename = "a")]
    pub args: Vec<String>,

    /// 选项，类似于--key=value的形式，不会保存“--”
    #[serde(rename = "opt")]
    pub opts: HashMap<String, Value>,

    /// 操作来源，可能是系统生成的
    /// - 例如 arv vault remove aaa，会导致生成将aaa中的对象move到默认库的操作
    #[serde(rename = "sc")]
    pub source: OperSource,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum OperSource {
    User = 1,
    System = 2,
}

impl Operation {
    pub fn simple(main: &str, args: Vec<String>, opts: HashMap<String, Value>) -> Self {
        Self {
            main: main.to_string(),
            sub: String::new(),
            directive: String::new(),
            args,
            opts,
            source: OperSource::User,
        }
    }

    pub fn new(
        main: &str,
        sub: &str,
        directive: &str,
        args: Vec<String>,
        opts: HashMap<String, Value>,
    ) -> Self {
        Self {
            main: main.to_string(),
            sub: sub.to_string(),
            directive: directive.to_string(),
            args,
            opts,
            source: OperSource::User,
        }
    }

    pub fn system(
        main: &str,
        sub: &str,
        directive: &str,
        args: Vec<String>,
        opts: HashMap<String, Value>,
    ) -> Self {
        Self {
            main: main.to_string(),
            sub: sub.to_string(),
            directive: directive.to_string(),
            args,
            opts,
            source: OperSource::System,
        }
    }

    pub fn to_display(&self) -> String {
        let main = match self.main.as_str() {
            short::main::PUT => self.main.bright_green().to_string(),
            short::main::RESTORE => self.main.orange().to_string(),
            short::main::MOVE => self.main.cyan().to_string(),
            short::main::VAULT => self.main.purple().to_string(),
            short::main::CONFIG => self.main.yellow().to_string(),
            short::main::UPDATE => self.main.magenta().to_string(),
            _ => self.main.clone(), // 默认不变
        };

        let mut result: Vec<String> = vec![main];
        if !self.sub.is_empty() {
            result.push(self.sub.bright_black().to_string());
        }

        if !self.args.is_empty() {
            result.append(self.args.clone().as_mut());
        }

        for (key, value) in &self.opts {
            let entry = match value {
                Value::String(s) => format!("--{}={}", key, s),
                Value::Bool(b) => {
                    if *b {
                        format!("--{}", key)
                    } else {
                        continue;
                    }
                }
                _ => continue, // 其他类型不处理
            };

            result.push(entry.dimmed_orange());
        }

        result.join(" ")
    }
}
