use crate::must_ok;

use chrono::NaiveDate;
use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::{fs, vec};

use crate::misc::{dt, paths};

fn default_current_vault_id() -> u32 {
    0
}

fn default_auto_check_update() -> String {
    "on".to_string()
}

fn default_last_check_update_date() -> NaiveDate {
    dt::now_d()
}

fn default_alias() -> Vec<AliasEntry> {
    vec![]
}

#[derive(Serialize, Deserialize)]
pub struct ArchiverConfig {
    #[serde(default = "default_current_vault_id")]
    /// 自动检查更新的开关，默认为on
    pub current_vault_id: u32,

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
            current_vault_id: 0,
            auto_check_update: "on".to_string(),
            last_check_update_date: dt::now_d(),
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

// pub static CONFIG: Lazy<ArchiverConfig> = Lazy::new(|| {
//     let content = must_ok!(
//         fs::read_to_string(paths::CONFIG_FILE_PATH.as_path()),
//         "Cannot read config file"
//     );

//     must_ok!(
//         serde_json::from_str::<ArchiverConfig>(&content),
//         "Cannot parse config file"
//     )
// });

pub const CONFIG_ITEMS: [&str; 2] = ["alias", "auto-check-update"];
