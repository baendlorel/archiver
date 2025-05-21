#[derive(Clone)]
pub struct StackFrame {
    pub file: &'static str,
    pub line: u32,
    pub col: u32,
    pub module_path: &'static str,
}

pub enum ArchiverErrorLevel {
    Fatal,
    Warn,
    Info,
}

pub struct ArchiverError {
    pub level: ArchiverErrorLevel,
    pub message: String,
    pub stack: Vec<StackFrame>,
}

impl ArchiverError {
    pub fn info(message: String, stack: Vec<StackFrame>) -> Self {
        Self {
            message,
            stack,
            level: ArchiverErrorLevel::Info,
        }
    }

    pub fn warn(message: String, stack: Vec<StackFrame>) -> Self {
        Self {
            message,
            stack,
            level: ArchiverErrorLevel::Warn,
        }
    }

    pub fn fatal(message: String, stack: Vec<StackFrame>) -> Self {
        Self {
            message,
            stack,
            level: ArchiverErrorLevel::Fatal,
        }
    }
}

impl std::fmt::Display for ArchiverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.level {
            ArchiverErrorLevel::Info => return write!(f, "{}", self.message),
            ArchiverErrorLevel::Warn => return write!(f, "{}", self.message),
            ArchiverErrorLevel::Fatal => {
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
                write!(
                    f,
                    "{} - {} \n{}",
                    "Fatal",
                    self.message,
                    stack_info.join("\n")
                )
            }
        }
    }
}

#[macro_export]
macro_rules! println_err {
    ($e:expr) => {
        match $e.level {
            crate::models::error::ArchiverErrorLevel::Fatal => {
                println!("{}", $e.to_string().red());
            }
            crate::models::error::ArchiverErrorLevel::Warn => {
                println!("{}", $e.to_string().yellow());
            }
            crate::models::error::ArchiverErrorLevel::Info => {
                println!("{}", $e.to_string());
            }
        }
    };
}

#[macro_export]
macro_rules! err {
    ($e:expr) => {
        $crate::models::error::ArchiverError::fatal(
            $e.to_string(),
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
macro_rules! err_info {
    ($e:expr) => {
        $crate::models::error::ArchiverError::info(
            $e.to_string(),
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
macro_rules! err_warn {
    ($e:expr) => {
        $crate::models::error::ArchiverError::warn(
            $e.to_string(),
            vec![$crate::models::error::StackFrame {
                file: file!(),
                line: line!(),
                col: column!(),
                module_path: module_path!(),
            }],
        )
    };
}

// wrap_expect!(
//     Err(2) as Result<(), i32>,
//     "Failed to create auto increment file"
// );
#[macro_export]
macro_rules! wrap_expect {
    ($e:expr, $s:expr) => {
        $e.expect(
            format!(
                "{}\n  at {} {}:{} ({})",
                $s,
                file!(),
                line!(),
                column!(),
                module_path!()
            )
            .as_str(),
        )
    };
}

#[macro_export]
macro_rules! wrap_err {
    ($o:expr) => {
        match $o {
            Ok(val) => Ok(val),
            Err(e) => Err($crate::err!(e)),
        }
    };
}

// #[macro_export]
// macro_rules! wrap_self {
//     ($e:expr) => {
//         return $crate::models::error::ArchiverError {
//             message: $e.message,
//             stack: {
//                 let mut stack = $e.stack.clone();
//                 stack.push($crate::models::error::StackFrame {
//                     file: file!(),
//                     line: line!(),
//                     col: column!(),
//                     module_path: module_path!(),
//                 });
//                 stack
//             },
//             level: $e.level,
//         };
//     };
// }

#[macro_export]
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
