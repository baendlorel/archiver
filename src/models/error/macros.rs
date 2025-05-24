// todo 缩减、归并宏

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
                error.to_string(),
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
/// 包裹一个Option对象，手动添加stack
macro_rules! wrap_option_err_fatal {
    ($o:expr, $e:expr) => {
        match $o {
            Some(val) => Ok(val),
            None => Err($crate::err_fatal_from_str!("{}", $e.to_string())),
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
                Err($crate::models::error::ArchiverError {
                    message: e.message,
                    stack,
                    level: e.level,
                })
            }
        }
    };
}

#[macro_export]
/// 展示一个ArchiverError的错误，但只是看看，没关系，继续执行后面的
macro_rules! log_if_err {
    ($e:expr) => {
        $e.map_err(|e| e.display()).ok()
    };
}
