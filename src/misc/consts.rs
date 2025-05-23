use once_cell::sync::Lazy;
use owo_colors::OwoColorize;

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
