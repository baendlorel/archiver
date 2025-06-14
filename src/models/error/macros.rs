/// 此处的宏无法再缩减，err_fatal在下面还有其他地方用
/// 故同理，另外两个warn/info宏也无法再缩减

/// 创建一个error级别的ArchiverError，支持字符串模板
#[macro_export]
macro_rules! err_error {
    ($($arg:tt)*) => {
        $crate::models::error::ArchiverError::error(
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

/// 创建一个warn级别的ArchiverError的Result返回，支持字符串模板
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        Err($crate::err_error!($($arg)*))
    };
}

/// 创建一个fatal级别的ArchiverError，支持字符串模板
#[macro_export]
macro_rules! err_fatal {
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

/// 创建一个fatal级别的ArchiverError的Result返回，支持字符串模板
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        Err($crate::err_fatal!($($arg)*))
    };
}

/// 创建一个info级别的ArchiverError，支持字符串模板
#[macro_export]
macro_rules! err_info {
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

/// 创建一个info级别的ArchiverError的Result返回，支持字符串模板
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        Err($crate::err_info!($($arg)*))
    };
}

/// 创建一个warn级别的ArchiverError，支持字符串模板
#[macro_export]
macro_rules! err_warn {
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

/// 创建一个warn级别的ArchiverError的Result返回，支持字符串模板
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        Err($crate::err_warn!($($arg)*))
    };
}

/// 包裹不允许失败的Result对象
/// - 以unwrap_or_else处理
/// - 为Err会直接panic
#[macro_export]
macro_rules! must_ok {
    ($e:expr) => {
        $e.unwrap_or_else(|error| {
            panic!(
                "{} {}\n at {} {}:{}",
                crate::misc::clap_mark::fatal(),
                error.to_string(),
                file!(),
                line!(),
                column!()
            )
        })
    };
    ($e:expr, $s:expr) => {
        $e.unwrap_or_else(|error| {
            panic!(
                "{} {}{}\n at {} {}:{}",
                crate::misc::clap_mark::fatal(),
                if $s.is_empty() {
                    String::new()
                } else {
                    format!("{}\n", $s)
                },
                error.to_string(),
                file!(),
                line!(),
                column!()
            )
        })
    };
}

/// 包裹不允许为None的Option对象
/// - 以unwrap_or_else处理
/// - 为None会直接panic
#[macro_export]
macro_rules! must_some {
    ($e:expr, $s:expr) => {
        $e.unwrap_or_else(|| {
            panic!(
                "{} {}\n at {} {}:{}",
                crate::misc::clap_mark::fatal(),
                $s,
                file!(),
                line!(),
                column!()
            )
        })
    };
}

/// 包裹Result或Option，并叠加stack，支持：
/// - 不含ArchiverError的Result对象
/// - Option对象（需第二个参数message）
#[macro_export]
macro_rules! as_fatal {
    ($o:expr) => {
        match $o {
            Ok(val) => Ok(val),
            Err(e) => Err($crate::err_fatal!("{}", e.to_string())),
        }
    };
    ($o:expr, $e:expr) => {
        match $o {
            Some(val) => Ok(val),
            None => Err($crate::err_fatal!("{}", $e.to_string())),
        }
    };
}

/// 包裹Result<_,ArchiverError>，叠加stack
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
                Err($crate::models::error::ArchiverError {
                    message: e.message,
                    stack,
                    level: e.level,
                })
            }
        }
    };
}
