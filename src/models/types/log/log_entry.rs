use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use strip_ansi_escapes::strip_str;

use super::LogLevel;
use crate::cli::{Operation, short};
use crate::core::{config, vault};
use crate::misc::console::table::{TableRow, TableRowify};
use crate::misc::dt;
use crate::models::serde_custom::naive_date_time;
use crate::traits::CustomColors;

/// 定义用于序列化到JSON的日志条目结构
#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    #[serde(rename = "oat", with = "naive_date_time")]
    pub opered_at: NaiveDateTime, // 操作时间

    #[serde(rename = "lv")]
    pub level: LogLevel,

    #[serde(rename = "o")]
    pub oper: Operation, // 操作的完整信息

    #[serde(rename = "r")]
    pub remark: String, // 备注

    #[serde(rename = "aid", skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<u32>, // archive id，如果有的话

    #[serde(rename = "vid", skip_serializing_if = "Option::is_none")]
    pub vault_id: Option<u32>, // archive id，如果有的话
}

impl LogEntry {
    pub fn new(
        oper: Operation,
        level: LogLevel,
        remark: String,
        archive_id: Option<u32>,
        vault_id: Option<u32>,
    ) -> Self {
        Self {
            opered_at: dt::now_dt(),
            oper,
            level,
            remark: strip_str(remark),
            archive_id,
            vault_id,
        }
    }

    pub fn to_display(&self) -> String {
        let time = dt::to_dt_string(&self.opered_at);

        let archive_id = if let Some(archive_id) = self.archive_id {
            if self.oper.main == short::main::PUT {
                archive_id.to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let vault_name = if let Some(vault_id) = self.vault_id {
            match self.oper.main.as_str() {
                short::main::PUT => vault::get_name(vault_id),
                short::main::VAULT => vault::get_name(vault_id),
                _ => String::new(),
            }
        } else {
            String::new()
        };

        let remark = config::alias::apply(&self.remark)
            .replace("\n", "\\n")
            .to_string();

        let avid = match (archive_id.is_empty(), vault_name.is_empty()) {
            (true, true) => String::new(),
            (false, true) => format!(
                "({}{}{})",
                "-".styled_vault(),
                *config::VLT_ITEM_SEP,
                archive_id.styled_id(),
            ),
            (true, false) => format!(
                "({}{}{})",
                vault_name.styled_vault(),
                *config::VLT_ITEM_SEP,
                "-".styled_id(),
            ),
            (false, false) => format!(
                "({}{}{})",
                vault_name.styled_vault(),
                *config::VLT_ITEM_SEP,
                archive_id.styled_id(),
            ),
        };

        // 下面处理remark、archive_id和vault_name的显示
        let rav = match (self.remark.is_empty(), avid.is_empty()) {
            (true, true) => "".to_string(),
            (false, true) => remark.bright_black().to_string(),
            (true, false) => avid,
            (false, false) => format!("{} {}", remark.bright_black(), avid),
        };

        let second_dash = if !rav.is_empty() {
            if self.level.is_succ() {
                " - ".green().to_string()
            } else {
                " - ".red().to_string()
            }
        } else {
            String::new()
        };

        format!(
            "{} {} {}{second_dash}{}",
            time.bright_black(),
            self.level.to_mark(),
            self.oper.to_display(),
            rav,
            second_dash = second_dash
        )
    }
}

impl TableRowify for LogEntry {
    fn to_table_row(&self) -> crate::misc::console::table::TableRow {
        let mut cells = vec![
            dt::to_dt_string(&self.opered_at).bright_black().to_string(),
            self.level.to_mark(),
            self.oper.to_display(),
        ];
        let archive_id = if let Some(archive_id) = self.archive_id {
            if self.oper.main == short::main::PUT {
                archive_id.to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let vault_name = if let Some(vault_id) = self.vault_id {
            match self.oper.main.as_str() {
                short::main::PUT => vault::get_name(vault_id),
                short::main::VAULT => vault::get_name(vault_id),
                _ => String::new(),
            }
        } else {
            String::new()
        };

        let remark = config::alias::apply(&self.remark)
            .replace("\n", "\\n")
            .to_string();

        let avid = match (archive_id.is_empty(), vault_name.is_empty()) {
            (true, true) => String::new(),
            (false, true) => format!(
                "({}{}{})",
                "-".styled_vault(),
                *config::VLT_ITEM_SEP,
                archive_id.styled_id(),
            ),
            (true, false) => format!(
                "({}{}{})",
                vault_name.styled_vault(),
                *config::VLT_ITEM_SEP,
                "-".styled_id(),
            ),
            (false, false) => format!(
                "({}{}{})",
                vault_name.styled_vault(),
                *config::VLT_ITEM_SEP,
                archive_id.styled_id(),
            ),
        };

        // 下面处理remark、archive_id和vault_name的显示
        let rav = match (self.remark.is_empty(), avid.is_empty()) {
            (true, true) => "".to_string(),
            (false, true) => remark.bright_black().to_string(),
            (true, false) => avid,
            (false, false) => format!("{} {}", remark.bright_black(), avid),
        };

        cells.push(rav);

        TableRow::new(cells)
    }
}
