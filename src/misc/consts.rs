/// 标记符号，和LogLevel的名称一致
/// - 不放在LogLevel是因为可能在其他地方单独用它
pub mod mark {
    use owo_colors::OwoColorize;

    pub fn succ() -> String {
        "✓".green().bold().to_string()
    }

    pub fn error() -> String {
        "!".red().bold().to_string()
    }

    pub fn fatal() -> String {
        "✗".bright_red().bold().to_string()
    }

    pub fn warn() -> String {
        "⚠".yellow().to_string()
    }

    pub fn info() -> String {
        "i".cyan().bold().to_string()
    }
}

pub mod clap_mark {
    use owo_colors::OwoColorize;

    pub fn succ() -> String {
        " succ:".green().bold().to_string()
    }

    pub fn error() -> String {
        "error:".red().bold().to_string()
    }

    pub fn fatal() -> String {
        "fatal:".bright_red().bold().underline().to_string()
    }

    pub fn warn() -> String {
        " warn:".yellow().to_string()
    }

    pub fn info() -> String {
        " info:".cyan().bold().to_string()
    }
}
