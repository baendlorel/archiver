pub mod mark {
    use owo_colors::OwoColorize;

    pub fn succ() -> String {
        "✓".green().to_string()
    }

    pub fn fail() -> String {
        "✗".red().to_string()
    }

    pub fn warn() -> String {
        "⚠".yellow().to_string()
    }

    pub fn info() -> String {
        "i".cyan().to_string()
    }
}
