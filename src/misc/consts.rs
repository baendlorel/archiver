use once_cell::sync::Lazy;
use owo_colors::OwoColorize;

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

pub const CONFIG_HELP_TEXT: &str = r#"Show configs:
  arv config           # show all configs
  arv config <item>    # show the specified config item

Set config item to some value: 
  arv config <item>.<directive> <value>
e.g.
  alias.add <alias>    # <alias> is like `my=/dir/temp`
  alias.remove <alias>
  auto-check-update.set on/off"#;

pub const CONFIG_VALID_STMT: Lazy<String> = Lazy::new(|| {
    let valid_stmt = r#"
  alias.add <alias>    # <alias> is like `my=/dir/temp`
  alias.remove <alias>
  auto-check-update.set on/off"#;

    format!("{}{}", "Valid Statement".green(), valid_stmt)
});
