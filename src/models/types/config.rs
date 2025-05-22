use std::vec;

use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ArchiverConfig {
    pub auto_check_update: String,
    /// 别名映射表
    pub alias: Vec<AliasEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct AliasEntry {
    pub alias: String,
    pub origin: String,
}

impl ArchiverConfig {
    pub fn display(&self, item: &str) -> String {
        match item {
            "alias" => {
                let head = "Alias".fg_rgb::<153, 153, 153>();
                let mut result: Vec<String> = vec![head.to_string()];
                result.push(format!(
                    "  ~={} {}",
                    crate::misc::paths::HOME_DIR.to_string_lossy(),
                    "(default)".cyan()
                ));
                for entry in &self.alias {
                    result.push(format!("  {}={}", entry.alias, entry.origin));
                }
                result.join("\n")
            }
            "auto-check-update" => {
                let head = "AutoCheckUpdate (after each command)".fg_rgb::<153, 153, 153>();
                let status = if self.auto_check_update == "on" {
                    "on".green().to_string()
                } else {
                    "off".red().to_string()
                };
                format!("{}\n  {}", head, status)
            }
            _ => format!(
                "Undefined config item '{}'\nValid config items: {}",
                item,
                CONFIG_ITEMS.join(", ")
            ),
        }
    }
}

pub const CONFIG_ITEMS: [&str; 2] = ["alias", "auto-check-update"];
