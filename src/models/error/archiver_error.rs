use owo_colors::OwoColorize;
use strip_ansi_escapes::strip_str;

pub enum ArchiverErrorLevel {
    Fatal,
    Warn,
    Info,
}

// 下面是ArchiverError用的错误等级
const FATAL: &str = "!"; // \u{2757};
const WARN: &str = "⚠";
const INFO: &str = "i"; // \u{2139}

impl ArchiverErrorLevel {
    pub fn to_string_styled(&self) -> String {
        match self {
            ArchiverErrorLevel::Info => format!("{} {}", INFO, "Info")
                .cyan()
                .underline()
                .to_string(),
            ArchiverErrorLevel::Warn => format!("{} {}", WARN, "Warn")
                .yellow()
                .underline()
                .to_string(),
            ArchiverErrorLevel::Fatal => format!("{} {}", FATAL, "Fatal")
                .red()
                .underline()
                .to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ArchiverErrorLevel::Info => format!("{}", "Info"),
            ArchiverErrorLevel::Warn => format!("{}", "Warn"),
            ArchiverErrorLevel::Fatal => format!("{}", "Fatal"),
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
    pub level: ArchiverErrorLevel,
    pub message: String,
    pub stack: Vec<StackFrame>,
}

pub type ArchiverResult<T> = Result<T, ArchiverError>;

impl ArchiverError {
    fn new(level: ArchiverErrorLevel, message: String, stack: Vec<StackFrame>) -> Self {
        // 听从建议，给每个错误信息加上句号
        let message = format!("{}.", message.trim_end_matches("."));
        Self {
            level,
            message,
            stack,
        }
    }

    pub fn info(message: String, stack: Vec<StackFrame>) -> Self {
        Self::new(ArchiverErrorLevel::Info, message, stack)
    }

    pub fn warn(message: String, stack: Vec<StackFrame>) -> Self {
        Self::new(ArchiverErrorLevel::Warn, message, stack)
    }

    pub fn fatal(message: String, stack: Vec<StackFrame>) -> Self {
        Self::new(ArchiverErrorLevel::Fatal, message, stack)
    }

    // pub fn set_message(&mut self, message: String) {
    //     self.message = format!("{}.", message.trim_end_matches("."));
    // }

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
    fn to_log(&self) -> String {
        format!("{}\n{}", self.message, self.get_stack_string())
    }

    #[cfg(not(feature = "dev"))]
    /// 将Error转化为显示在终端的日志，含彩色
    /// - 生产环境下，仅fatal报错展示stack信息
    fn to_log(&self) -> String {
        match self.level {
            ArchiverErrorLevel::Fatal => format!("{}\n{}", self.message, self.get_stack_string()),
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
        let message = strip_str(self.message.as_str());
        let stack = self.get_stack_string();
        format!("{} - {}\n{}", self.level.to_string(), message, stack)
    }

    #[cfg(not(feature = "dev"))]
    /// 将Error转化为写入文件的字符串，无彩色
    /// - 生产环境下，仅fatal报错包含stack信息
    pub fn to_string(&self) -> String {
        let message = strip_str(self.message.as_str());
        match self.level {
            ArchiverErrorLevel::Fatal => {
                let stack_info = self.get_stack_string();
                return format!("{} - {}\n{}", self.level.to_string(), message, stack_info);
            }
            _ => return format!("{} - {}", self.level.to_string(), message),
        }
    }
}
