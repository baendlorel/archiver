pub mod status_mark {
    use once_cell::sync::Lazy;
    use owo_colors::OwoColorize;

    static FAIL: Lazy<String> = Lazy::new(|| "✗".red().to_string());
    static SUCC: Lazy<String> = Lazy::new(|| "✓".green().to_string());
    static WARN: Lazy<String> = Lazy::new(|| "⚠".yellow().to_string());
    // static INFO: Lazy<String> = Lazy::new(|| "ℹ".cyan().to_string());

    pub fn succ() -> String {
        SUCC.clone()
    }

    pub fn fail() -> String {
        FAIL.clone()
    }

    pub fn warn() -> String {
        WARN.clone()
    }

    // pub fn info() -> String {
    //     INFO.clone()
    // }
}
