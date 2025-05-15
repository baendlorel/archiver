pub struct StackFrame {
    pub file: &'static str,
    pub line: u32,
    pub col: u32,
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
        for frame in &self.stack {
            stack_info.push_str(&format!("at {}:{}:{}\n", frame.file, frame.line, frame.col));
        }
        write!(f, "{}\n{}", self.message, stack_info)
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
            }],
        )
    };
}

#[macro_export]
macro_rules! werr {
    ($r:expr) => {
        r.map_err(|e| $crate::err!(e))
    };
}

// #[macro_export]
// macro_rules! update_error {
//     ($e:expr) => {{
//         let mut stack = $e.stack.clone();
//         stack.push(StackFrame {
//             file: file!(),
//             line: line!(),
//             col: column!(),
//             func: std::intrinsics::function_name!(),
//         });
//         ArchiverError::new($e.message, stack)
//     }};
// }

#[macro_export]
macro_rules! w {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let mut stack = e.stack.clone();
                stack.push($crate::models::error::StackFrame {
                    file: file!(),
                    line: line!(),
                    col: column!(),
                    func: std::intrinsics::function_name!(),
                });
                return Err($crate::models::error::ArchiverError {
                    message: e.message,
                    stack,
                });
            }
        }
    };
}
