use std::vec;

use chrono::{Local, NaiveDate};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

fn default_auto_check_update() -> String {
    "on".to_string()
}

fn default_last_check_update_date() -> NaiveDate {
    Local::now().date_naive()
}

fn default_alias() -> Vec<AliasEntry> {
    vec![]
}

#[derive(Serialize, Deserialize)]
pub struct ArchiverConfig {
    #[serde(default = "default_auto_check_update")]
    /// 自动检查更新的开关，默认为on
    pub auto_check_update: String,

    /// 上次检查更新的时间
    #[serde(default = "default_last_check_update_date")]
    pub last_check_update_date: NaiveDate,

    /// 别名映射表
    #[serde(default = "default_alias")]
    pub alias: Vec<AliasEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct AliasEntry {
    pub alias: String,
    pub origin: String,
}

impl ArchiverConfig {
    pub fn default() -> ArchiverConfig {
        Self {
            auto_check_update: "on".to_string(),
            last_check_update_date: Local::now().date_naive(),
            alias: vec![],
        }
    }

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
