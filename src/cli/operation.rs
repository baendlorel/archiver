use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::cli::short;
use crate::traits::CustomColors;

/// 完整操作信息，包含主命令、子命令、指令、参数和选项等
#[derive(Serialize, Deserialize, Clone)]
pub struct Operation {
    /// 主命令，比如put、restore
    #[serde(rename = "m")]
    pub main: String,

    /// 子命令，比如vault的create、use
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,

    /// 指令，比如config alias add的add
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub directive: Option<String>,

    /// 参数，直接跟在主/子命令后面的
    /// - 有了子命令时，主命令不会有参数
    #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    /// 选项，类似于--key=value的形式，不会保存“--”
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub opts: Option<HashMap<String, Value>>,

    /// 操作来源，可能是系统生成的
    /// - 例如 arv vault remove aaa，会导致生成将aaa中的对象move到默认库的操作
    #[serde(rename = "sc")]
    pub source: OperSource,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum OperSource {
    #[serde(rename = "u")]
    User,

    #[serde(rename = "s")]
    System = 2,
}

impl Operation {
    pub fn simple(
        main: &str,
        args: Option<Vec<String>>,
        opts: Option<HashMap<String, Value>>,
    ) -> Self {
        Self {
            main: main.to_string(),
            sub: None,
            directive: None,
            args,
            opts,
            source: OperSource::User,
        }
    }

    pub fn new(
        main: &str,
        sub: Option<&str>,
        directive: Option<&str>,
        args: Option<Vec<String>>,
        opts: Option<HashMap<String, Value>>,
    ) -> Self {
        Self {
            main: main.to_string(),
            sub: sub.map(|s| s.to_string()),
            directive: directive.map(|d| d.to_string()),
            args,
            opts,
            source: OperSource::User,
        }
    }

    pub fn sys(
        main: &str,
        sub: Option<&str>,
        directive: Option<&str>,
        args: Option<Vec<String>>,
        opts: Option<HashMap<String, Value>>,
    ) -> Self {
        Self {
            main: main.to_string(),
            sub: sub.map(|s| s.to_string()),
            directive: directive.map(|d| d.to_string()),
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
        if let Some(sub) = &self.sub {
            result.push(sub.bright_black().to_string());
        }

        if let Some(args) = &self.args {
            result.append(args.clone().as_mut());
        }

        if let Some(opts) = &self.opts {
            for (key, value) in opts {
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
        }

        result.join(" ")
    }
}
