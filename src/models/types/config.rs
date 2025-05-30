use crate::map;

use chrono::NaiveDate;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::vec;

use crate::misc::{dt, paths};
use crate::traits::{CustomColors, ForceToString};

#[derive(Serialize, Deserialize, Clone)]
pub struct ArchiverConfig {
    /// 自动检查更新的开关，默认为on
    pub current_vault_id: u32,

    /// 自动检查更新的开关，默认为on
    pub auto_check_update: String,

    /// 上次检查更新的时间
    pub last_check_update_date: NaiveDate,

    /// 别名映射表
    pub alias_map: std::collections::HashMap<String, String>,

    /// vault item的分隔符，默认为冒号
    /// - 如果vault名为`@`，归档项名字为`temp`，则输出格式为`@:temp`
    /// - 会影响
    ///     - ListEntry的显示
    ///     - LogEntry的显示
    pub vault_item_seperator: String,
}

impl ArchiverConfig {
    pub fn default() -> ArchiverConfig {
        Self {
            current_vault_id: 0,
            auto_check_update: "on".to_string(),
            last_check_update_date: dt::now_d(),
            alias_map: map![],
            vault_item_seperator: ":".to_string(),
        }
    }

    pub fn display(&self) {
        // 保留map以供未来扩展
        let m = vec![
            (
                "root",
                "ArchiverPath",
                paths::ROOT_DIR.force_to_string().styled_const(),
            ),
            ("alias", "Alias", {
                // 按键排序后遍历
                let mut aliases: Vec<String> = vec![];
                let mut max_alias_width = 0;
                self.alias_map
                    .iter()
                    .for_each(|(alias, _)| max_alias_width = max_alias_width.max(alias.len()));
                for (alias, origin) in &self.alias_map {
                    let styled = if alias == "~" {
                        format!(
                            "{}{} => {}",
                            alias.styled_const(),
                            " ".repeat(max_alias_width - alias.len()),
                            origin.styled_const()
                        )
                    } else {
                        format!(
                            "{}{} => {}",
                            alias.styled_json_string(),
                            " ".repeat(max_alias_width - alias.len()),
                            origin.styled_json_string()
                        )
                    };
                    aliases.push(styled);
                }
                aliases.sort_by(|a, b| {
                    use crate::traits::StripAnsi;
                    // 去除 ANSI 代码后再比较
                    let a_clean = a.strip_ansi();
                    let b_clean = b.strip_ansi();

                    // ~ 别名排在最前面
                    if a_clean.starts_with('~') && !b_clean.starts_with('~') {
                        std::cmp::Ordering::Less
                    } else if !a_clean.starts_with('~') && b_clean.starts_with('~') {
                        std::cmp::Ordering::Greater
                    } else {
                        a_clean.cmp(&b_clean)
                    }
                });
                if aliases.is_empty() {
                    "{}".to_string()
                } else {
                    format!("{{\n  {}\n}}", aliases.join(",\n  "))
                }
            }),
            (
                "auto-check-update",
                "AutoCheckUpdate",
                if self.auto_check_update == "on" {
                    "on".green().to_string()
                } else {
                    "off".red().to_string()
                },
            ),
            ("vault-item-seperator", "VaultItemSeperator", {
                format!("\"{}\"", self.vault_item_seperator).styled_json_string()
            }),
        ];

        m.iter().for_each(|(_, field, value)| {
            println!("{}: {}", field.styled_config_field(), value);
        });

        println!();
        println!(
            "{} means you cannot modify it.",
            "This color".styled_const().underline()
        );
    }
}
