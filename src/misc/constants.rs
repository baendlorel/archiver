pub mod status_mark {
    use once_cell::sync::Lazy;
    use owo_colors::OwoColorize;

    static FAIL: Lazy<String> = Lazy::new(|| "✗".red().to_string());
    static SUCC: Lazy<String> = Lazy::new(|| "✓".green().to_string());

    pub fn succ() -> String {
        SUCC.clone()
    }

    pub fn fail() -> String {
        FAIL.clone()
    }
}
