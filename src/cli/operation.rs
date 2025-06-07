use owo_colors::OwoColorize;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

use crate::cli::short;
use crate::models::serde_custom::opt;
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
    System,

    #[serde(rename = "t")]
    Transformed,
}

#[derive(Clone)]
pub enum Opt {
    /// 选项的值是字符串
    String(String),

    /// 选项的值是布尔值
    Bool(bool),

    /// 选项的值是布尔值
    U32(u32),
}

impl Operation {
    pub fn user(
        main: &str,
        sub: Option<String>,
        args: Option<Vec<String>>,
        opts: Option<HashMap<String, Opt>>,
    ) -> Self {
        Self {
            main: main.to_string(),
            sub,
            args,
            opts,
            source: OperSource::User,
        }
    }

    pub fn sys(
        main: &str,
        sub: Option<String>,
        args: Option<Vec<String>>,
        opts: Option<HashMap<String, Opt>>,
    ) -> Self {
        Self {
            main: main.to_string(),
            sub,
            args,
            opts,
            source: OperSource::System,
        }
    }

    pub fn trans(
        main: &str,
        sub: Option<String>,
        args: Option<Vec<String>>,
        opts: Option<HashMap<String, Opt>>,
    ) -> Self {
        Self {
            main: main.to_string(),
            sub,
            args,
            opts,
            source: OperSource::Transformed,
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
            result.push(sub.grey());
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
            result.push(sub.grey());
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
    // * 不带$args处理的情形
    ($main:expr) => {{ crate::cli::Operation::user($main, None, None, None) }};
    ($main:expr,None,$opts:expr) => {{ crate::cli::Operation::user($main, None, None, $opts) }};
    ($main:expr,$sub:expr,None,None) => {{
        use crate::traits::EnsureOption;
        crate::cli::Operation::user($main, $sub.ensure_option(), None, None)
    }};
    ($main:expr,$sub:expr,None,$opts:expr) => {{
        use crate::traits::EnsureOption;
        crate::cli::Operation::user($main, $sub.ensure_option(), None, $opts)
    }};

    // * 带$args处理的情形
    ($main:expr,$args:expr) => {{ oper!($main, None, $args, None) }};
    ($main:expr,$args:expr,$opts:expr) => {{ oper!($main, None, $args, $opts) }};
    ($main:expr,$sub:expr,$args:expr,$opts:expr) => {{ oper!($main, $sub, $args, $opts, "user") }};

    // & 完整参数，带有source区分
    ($main:expr,$sub:expr,$args:expr,$opts:expr,$src:expr) => {{
        use crate::cli::Operation;
        use crate::traits::EnsureOption;

        let mut args = vec![];
        for a in $args {
            if let Some(a) = a.clone().ensure_option() {
                args.push(format!("{}", a));
            }
        }

        match $src {
            "sys" => Operation::sys($main, $sub.ensure_option(), args.ensure_option(), $opts),
            "trans" => Operation::trans($main, $sub.ensure_option(), args.ensure_option(), $opts),
            "user" | _ => Operation::user($main, $sub.ensure_option(), args.ensure_option(), $opts),
        }
    }};
}

impl Serialize for Opt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        opt::serialize(self, serializer)
    }
}

impl<'de> Deserialize<'de> for Opt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        opt::deserialize(deserializer)
    }
}
