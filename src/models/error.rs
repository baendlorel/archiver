use owo_colors::OwoColorize;

pub enum Level {
    Fatal,
    Warn,
    Info,
}

// 下面是ArchiverError用的错误等级
const FATAL: &str = "!"; // \u{2757};
const WARN: &str = "⚠";
const INFO: &str = "i"; // \u{2139}

impl Level {
    pub fn to_string_styled(&self) -> String {
        match self {
            Level::Info => format!("{} {}", INFO, "Info")
                .cyan()
                .underline()
                .to_string(),
            Level::Warn => format!("{} {}", WARN, "Warn")
                .yellow()
                .underline()
                .to_string(),
            Level::Fatal => format!("{} {}", FATAL, "Fatal")
                .red()
                .underline()
                .to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Level::Info => format!("{}", "Info"),
            Level::Warn => format!("{}", "Warn"),
            Level::Fatal => format!("{}", "Fatal"),
        }
    }
}

#[derive(Clone)]
pub struct StackFrame {
    pub file: &'static str,
    pub line: u32,
    pub col: u32,
    pub module_path: &'static str,
}

pub struct ArchiverError {
    pub level: Level,
    pub message: String,
    pub stack: Vec<StackFrame>,
}

impl ArchiverError {
    pub fn info(message: String, stack: Vec<StackFrame>) -> Self {
        Self {
            message,
            stack,
            level: Level::Info,
        }
    }

    pub fn warn(message: String, stack: Vec<StackFrame>) -> Self {
        Self {
            message,
            stack,
            level: Level::Warn,
        }
    }

    pub fn fatal(message: String, stack: Vec<StackFrame>) -> Self {
        Self {
            message,
            stack,
            level: Level::Fatal,
        }
    }

    fn get_stack_string(&self) -> String {
        let mut stack_info: Vec<String> = vec![];
        let mut counter: u32 = 0;
        for frame in &self.stack {
            counter += 1;
            stack_info.push(format!(
                "  {}.at {}:{}:{} {}",
                counter,
                frame.file,
                frame.line,
                frame.col,
                frame.module_path.repeat(0) // 模块路径太长了，和普通路径重复，此处省略 frame.module_path
            ));
        }
        stack_info.join("\n")
    }

    #[cfg(feature = "dev")]
    /// 将Error转化为显示在终端的日志，含彩色
    /// - dev环境下总是显示stack信息
    fn to_log(&self) -> String {
        format!("{}\n{}", self.message, self.get_stack_string())
    }

    #[cfg(not(feature = "dev"))]
    /// 将Error转化为显示在终端的日志，含彩色
    /// - 生产环境下，仅fatal报错展示stack信息
    fn to_log(&self) -> String {
        match self.level {
            Level::Fatal => format!("{}\n{}", self.message, self.get_stack_string()),
            _ => self.message.clone(),
        }
    }

    pub fn display(&self) {
        println!("{} {}", self.level.to_string_styled(), self.to_log());
    }

    #[cfg(feature = "dev")]
    /// 将Error转化为写入文件的字符串，无彩色
    /// - dev环境下，包含全部stack信息
    pub fn to_string(&self) -> String {
        let stack_info = self.get_stack_string();
        format!(
            "{} - {}\n{}",
            self.level.to_string(),
            self.message,
            stack_info
        )
    }

    #[cfg(not(feature = "dev"))]
    /// 将Error转化为写入文件的字符串，无彩色
    /// - 生产环境下，仅fatal报错包含stack信息
    pub fn to_string(&self) -> String {
        match self.level {
            Level::Fatal => {
                let stack_info = self.get_stack_string();
                return format!(
                    "{} - {}\n{}",
                    self.level.to_string(),
                    self.message,
                    stack_info
                );
            }
            _ => return format!("{} - {}", self.level.to_string(), self.message),
        }
    }
}

#[macro_export]
/// 创建一个fatal级别的ArchiverError，支持字符串模板
macro_rules! err_fatal_from_str {
    ($($arg:tt)*) => {
        $crate::models::error::ArchiverError::fatal(
            format!($($arg)*),
            vec![$crate::models::error::StackFrame {
                file: file!(),
                line: line!(),
                col: column!(),
                module_path: module_path!(),
            }],
        )
    };
}

#[macro_export]
/// 创建一个fatal级别的ArchiverError的Result返回，支持字符串模板
macro_rules! err_fatal {
    ($($arg:tt)*) => {
        Err($crate::err_fatal_from_str!($($arg)*))
    };
}

#[macro_export]
/// 创建一个info级别的ArchiverError，支持字符串模板
macro_rules! err_info_from_str {
    ($($arg:tt)*) => {
        $crate::models::error::ArchiverError::info(
            format!($($arg)*),
            vec![$crate::models::error::StackFrame {
                file: file!(),
                line: line!(),
                col: column!(),
                module_path: module_path!(),
            }],
        )
    };
}

#[macro_export]
/// 创建一个info级别的ArchiverError的Result返回，支持字符串模板
macro_rules! err_info {
    ($($arg:tt)*) => {
        Err($crate::err_info_from_str!($($arg)*))
    };
}

#[macro_export]
/// 创建一个warn级别的ArchiverError，支持字符串模板
macro_rules! err_warn_from_str {
    ($($arg:tt)*) => {
        $crate::models::error::ArchiverError::warn(
            format!($($arg)*),
            vec![$crate::models::error::StackFrame {
                file: file!(),
                line: line!(),
                col: column!(),
                module_path: module_path!(),
            }],
        )
    };
}

#[macro_export]
/// 创建一个warn级别的ArchiverError的Result返回，支持字符串模板
macro_rules! err_warn {
    ($($arg:tt)*) => {
        Err($crate::err_warn_from_str!($($arg)*))
    };
}

#[macro_export]
/// 包裹一个Result对象，让其在触发expect报错的时候可以附带代码位置
macro_rules! uoe_result {
    ($e:expr, $s:expr) => {
        $e.unwrap_or_else(|error| {
            panic!(
                "{} {}{}\n at {} {}:{}",
                crate::misc::mark::fail(),
                if $s.is_empty() {
                    "".to_string()
                } else {
                    format!("{}\n", $s)
                },
                error,
                file!(),
                line!(),
                column!()
            )
        })
    };
}

#[macro_export]
/// 包裹一个Option对象，让其在触发expect的时候可以附带代码位置
macro_rules! uoe_option {
    ($e:expr, $s:expr) => {
        $e.unwrap_or_else(|| {
            panic!(
                "{} {}\n at {} {}:{}",
                crate::misc::mark::fail(),
                $s,
                file!(),
                line!(),
                column!()
            )
        })
    };
}

#[macro_export]
/// 包裹一个不是ArchiverError的Result对象，手动添加stack
macro_rules! wrap_err_fatal {
    ($o:expr) => {
        match $o {
            Ok(val) => Ok(val),
            Err(e) => Err($crate::err_fatal_from_str!("{}", e.to_string())),
        }
    };
}

#[macro_export]
/// 包裹一个Result<_,ArchiverError>对象，继承其stack
macro_rules! wrap_result {
    ($o:expr) => {
        match $o {
            Ok(val) => Ok(val),
            Err(e) => {
                let mut stack = e.stack.clone();
                stack.push($crate::models::error::StackFrame {
                    file: file!(),
                    line: line!(),
                    col: column!(),
                    module_path: module_path!(),
                });
                return Err($crate::models::error::ArchiverError {
                    message: e.message,
                    stack,
                    level: e.level,
                });
            }
        }
    };
}

#[macro_export]
/// 包裹一个Option对象，让其在触发expect的时候可以附带代码位置
macro_rules! log_if_err {
    ($e:expr) => {
        $e.map_err(|e| e.display()).ok()
    };
}

#[test]
fn test_error_display() {
    let e = err_fatal_from_str!("asdf");
    e.display();
    let e = err_warn_from_str!("asdf");
    e.display();
    let e = err_info_from_str!("asdf");
    e.display();
}
