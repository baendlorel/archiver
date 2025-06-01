use crate::core::config::alias;
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
    pub update_check: String,

    /// 上次检查更新的时间
    pub last_update_check: NaiveDate,

    /// 别名映射表
    pub alias_map: std::collections::HashMap<String, String>,

    /// vault item的分隔符，默认为冒号
    /// - 如果vault名为`@`，归档项名字为`temp`，则输出格式为`@:temp`
    /// - 会影响
    ///     - ListEntry的显示
    ///     - LogEntry的显示
    pub vault_item_sep: String,
}

impl ArchiverConfig {
    pub fn default() -> ArchiverConfig {
        Self {
            current_vault_id: 0,
            update_check: "on".to_string(),
            last_update_check: dt::now_d(),
            alias_map: map![],
            vault_item_sep: ":".to_string(),
        }
    }

    pub fn display(&self) {
        // 保留map以供未来扩展
        // 此为3元组的数组，依次是（字段名，字段值，注释文本）
        let m = vec![
            (
                "ArchiverPath",
                alias::apply(&paths::ROOT_DIR.force_to_string()).styled_const(),
                "Archived items, configs, etc. are here.",
            ),
            (
                "Alias",
                {
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
                                alias.styled_string_value(),
                                " ".repeat(max_alias_width - alias.len()),
                                origin.styled_string_value()
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
                },
                "Shorten the paths displayed.",
            ),
            (
                "UpdateCheck",
                if self.update_check == "on" {
                    "on".green().to_string()
                } else {
                    "off".red().to_string()
                },
                "Automically check updates.",
            ),
            (
                "VaultItemSep",
                { format!("\"{}\"", self.vault_item_sep).styled_string_value() },
                "Shows as {vault}{sep}{item}.",
            ),
        ];

        m.iter().for_each(|(field, value, comment)| {
            println!(
                "{}: {}  {}{}",
                field.styled_field(),
                value,
                "// ".styled_comment(),
                comment.styled_comment()
            );
        });

        println!();
        println!(
            "{} means you cannot modify it.",
            "This color".styled_const().underline()
        );
    }
}
