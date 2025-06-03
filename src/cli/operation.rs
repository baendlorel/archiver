use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
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

    /// 参数，直接跟在主/子命令后面的
    /// - 有了子命令时，主命令不会有参数
    #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    /// 选项，类似于--key=value的形式，不会保存“--”
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub opts: Option<HashMap<String, Opt>>,

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

#[derive(Serialize, Deserialize, Clone)]
pub enum Opt {
    /// 选项的值是字符串
    String(String),

    /// 选项的值是布尔值
    Bool(bool),

    /// 选项的值是布尔值
    U32(u32),
}

impl Operation {
    pub fn new<'a>(
        main: &str,
        sub: impl Into<Option<&'a str>>,
        args: impl Into<Option<Vec<String>>>,
        opts: impl Into<Option<HashMap<String, Opt>>>,
    ) -> Self {
        Self {
            main: main.to_string(),
            sub: sub.into().map(|s| s.to_string()),
            args: args.into(),
            opts: opts.into(),
            source: OperSource::User,
        }
    }

    pub fn sys<'a>(
        main: &str,
        sub: impl Into<Option<&'a str>>,
        args: impl Into<Option<Vec<String>>>,
        opts: impl Into<Option<HashMap<String, Opt>>>,
    ) -> Self {
        Self {
            main: main.to_string(),
            sub: sub.into().map(|s| s.to_string()),
            args: args.into(),
            opts: opts.into(),
            source: OperSource::System,
        }
    }

    fn get_main(&self) -> String {
        match self.main.as_str() {
            short::main::PUT => self.main.bright_green().to_string(),
            short::main::RESTORE => self.main.orange().to_string(),
            short::main::MOVE => self.main.cyan().to_string(),
            short::main::VAULT => self.main.purple().to_string(),
            short::main::CONFIG => self.main.yellow().to_string(),
            short::main::UPDATE => self.main.magenta().to_string(),
            _ => self.main.clone(), // 默认不变
        }
    }

    pub fn to_display(&self) -> String {
        // 不同的指令给不同的颜色，更好看
        let main = self.get_main();

        let mut result: Vec<String> = vec![main];
        if let Some(sub) = &self.sub {
            result.push(sub.bright_black().to_string());
        }

        if let Some(args) = &self.args {
            result.append(args.clone().as_mut());
        }

        if let Some(opts) = &self.opts {
            for (key, value) in opts {
                let k = key.chars().next().unwrap();
                let entry = match value {
                    Opt::String(s) => format!("-{}\"{}\"", k, s),
                    Opt::U32(u) => format!("-{}\"{}\"", k, u),
                    Opt::Bool(b) => {
                        if *b {
                            format!("-{}", k)
                        } else {
                            continue;
                        }
                    }
                };

                result.push(entry.dimmed_orange());
            }
        }

        result.join(" ")
    }

    pub fn to_detailed_display(&self) -> String {
        // 不同的指令给不同的颜色，更好看
        let main = self.get_main();

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
                    Opt::String(s) => format!("--{}=\"{}\"", key, s),
                    Opt::U32(u) => format!("--{}=\"{}\"", key, u),
                    Opt::Bool(b) => {
                        if *b {
                            format!("--{}", key)
                        } else {
                            continue;
                        }
                    }
                };

                result.push(entry.dimmed_orange());
            }
        }

        result.join(" ")
    }
}

/// 只有一个
#[macro_export]
macro_rules! oper {
    // 只有主命令
    ($main:expr) => {{ Operation::new($main, None, None, None) }};

    // 有主命令和参数
    ($main:expr,$args:expr) => {{
        use crate::traits::EnsureOption;
        let mut args = vec![];
        for a in $args {
            let cloned = a.clone();
            let optioned = cloned.ensure_option();
            if let Some(a) = optioned {
                args.push(format!("{}", a));
            }
        }
        Operation::new($main, None, args.ensure_option(), None)
    }};

    // 主命令、选项
    ($main:expr,None,$opts:expr) => {{
        Operation::new(
            $main, None, None,
            $opts, // 没有必要ensure_option，因为都是opt_map!宏生成的
        )
    }};

    // 主命令、参数、选项
    ($main:expr,$args:expr,$opts:expr) => {{
        use crate::traits::EnsureOption;
        let mut args = vec![];
        for a in $args {
            let cloned = a.clone();
            let optioned = cloned.ensure_option();
            if let Some(a) = optioned {
                args.push(format!("{}", a));
            }
        }
        Operation::new(
            $main,
            None,
            args.ensure_option(),
            $opts, // 没有必要ensure_option，因为都是opt_map!宏生成的
        )
    }};

    // 完整参数
    ($main:expr,$sub:expr,None,None) => {{ Operation::new($main, $sub, None, None) }};

    // 完整参数
    ($main:expr,$sub:expr,$args:expr,$opts:expr) => {{
        use crate::traits::EnsureOption;
        let mut args = vec![];
        for a in $args {
            let cloned = a.clone();
            let optioned = cloned.ensure_option();
            if let Some(a) = optioned {
                args.push(format!("{}", a));
            }
        }
        Operation::new(
            $main,
            $sub,
            args.ensure_option(),
            $opts, // 没有必要ensure_option，因为都是opt_map!宏生成的
        )
    }};

    // 完整参数
    ($main:expr,$sub:expr,$args:expr,$opts:expr,"sys") => {{
        use crate::traits::EnsureOption;
        let mut args = vec![];
        for a in $args {
            let cloned = a.clone();
            let optioned = cloned.ensure_option();
            if let Some(a) = optioned {
                args.push(format!("{}", a));
            }
        }
        Operation::sys(
            $main,
            $sub,
            args.ensure_option(),
            $opts, // 没有必要ensure_option，因为都是opt_map!宏生成的
        )
    }};
}
