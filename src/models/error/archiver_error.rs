use crate::models::types::LogLevel;

#[derive(Clone)]
pub struct StackFrame {
    pub file: &'static str,
    pub line: u32,
    pub col: u32,
    pub module_path: &'static str,
}

pub struct ArchiverError {
    pub level: LogLevel,
    pub message: String,
    pub stack: Vec<StackFrame>,
}

pub type ArchiverResult<T> = Result<T, ArchiverError>;

impl ArchiverError {
    fn new(level: LogLevel, message: String, stack: Vec<StackFrame>) -> Self {
        // 听从建议，给每个错误信息加上句号
        let message = format!("{}.", message.trim_end_matches("."));
        Self {
            level,
            message,
            stack,
        }
    }

    pub fn info(message: String, stack: Vec<StackFrame>) -> Self {
        Self::new(LogLevel::Info, message, stack)
    }

    pub fn warn(message: String, stack: Vec<StackFrame>) -> Self {
        Self::new(LogLevel::Warn, message, stack)
    }

    pub fn fatal(message: String, stack: Vec<StackFrame>) -> Self {
        Self::new(LogLevel::Fatal, message, stack)
    }

    fn get_stack_string(&self) -> String {
        let mut stack_info: Vec<String> = vec![];
        let mut count: u32 = 0;
        for frame in &self.stack {
            count += 1;
            stack_info.push(format!(
                "  {}.at {}:{}:{} {}",
                count,
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
    pub fn to_string(&self) -> String {
        format!("{}\n{}", self.message, self.get_stack_string())
    }

    #[cfg(not(feature = "dev"))]
    /// 将Error转化为显示在终端的日志，含彩色
    /// - 生产环境下，仅fatal报错展示stack信息
    pub fn to_string(&self) -> String {
        match self.level {
            LogLevel::Fatal => format!("{}\n{}", self.message, self.get_stack_string()),
            _ => self.message.clone(),
        }
    }

    pub fn display(&self) {
        println!("{} {}", self.level.to_mark(), self.to_string());
    }
}
