#[derive(Clone)]
pub struct StackFrame {
    pub file: &'static str,
    pub line: u32,
    pub col: u32,
    pub module_path: &'static str,
}

pub struct ArchiverError {
    pub message: String,
    pub stack: Vec<StackFrame>,
}

impl ArchiverError {
    pub fn new(message: String, stack: Vec<StackFrame>) -> Self {
        Self { message, stack }
    }
}

impl std::fmt::Display for ArchiverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stack_info = String::new();
        let mut counter: u32 = 0;
        for frame in &self.stack {
            counter += 1;
            stack_info.push_str(&format!(
                " {}. at {}:{}:{} ({})\n",
                counter, frame.file, frame.line, frame.col, frame.module_path
            ));
        }
        write!(f, "{}\n{}", self.message, stack_info.trim_end_matches("\n"))
    }
}

#[macro_export]
macro_rules! err {
    ($e:expr) => {
        $crate::models::error::ArchiverError::new(
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
macro_rules! wrap_err {
    ($o:expr) => {
        match $o {
            Ok(val) => Ok(val),
            Err(e) => Err($crate::err!(e)),
        }
    };
}

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
                });
            }
        }
    };
}
