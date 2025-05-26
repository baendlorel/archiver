use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::field_style::CustomColors;

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
    #[serde(rename = "o")]
    pub opts: HashMap<String, Value>,
}

impl Operation {
    pub fn simple(main: &str, args: Vec<String>, opts: HashMap<String, Value>) -> Self {
        Self {
            main: main.to_string(),
            sub: String::new(),
            directive: String::new(),
            args: args,
            opts,
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
            directive: directive.to_string(),
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

    pub fn to_styled_string(&self) -> String {
        let mut result: Vec<String> = vec![self.main.yellow().to_string()];
        if !self.sub.is_empty() {
            result.push(self.sub.grey());
        }

        if !self.args.is_empty() {
            result.append(self.args.clone().as_mut());
        }

        for (key, value) in &self.opts {
            let key_colored = format!("--{}=", key).grey();
            result.push(format!("{}{}", key_colored, value.blue()));
        }

        result.join(" ")
    }
}
