use crate::{kv_row, must_ok};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::LogLevel;
use crate::cli::short::main;
use crate::cli::{OperSource, Operation};
use crate::core::{archive, auto_incr, config, vault};
use crate::misc::console::table::{Column, Table, TableRow, TableRowify};
use crate::misc::dt;
use crate::models::serde_custom::naive_date_time;
use crate::traits::CustomColors;

/// 定义用于序列化到JSON的日志条目结构
#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    pub id: u32, // 日志条目的唯一ID

    #[serde(rename = "oat", with = "naive_date_time")]
    pub opered_at: NaiveDateTime, // 操作时间

    #[serde(rename = "lv")]
    pub level: LogLevel,

    #[serde(rename = "o")]
    pub oper: Operation, // 操作的完整信息

    #[serde(rename = "m")]
    pub message: String, // 备注

    #[serde(rename = "aid", skip_serializing_if = "Option::is_none")]
    pub archive_ids: Option<Vec<u32>>, // archive id，如果有的话

    #[serde(rename = "vid", skip_serializing_if = "Option::is_none")]
    pub vault_ids: Option<Vec<u32>>, // archive id，如果有的话
}

impl LogEntry {
    pub fn new(
        oper: Operation,
        level: LogLevel,
        message: String,
        archive_ids: Option<Vec<u32>>,
        vault_ids: Option<Vec<u32>>,
    ) -> Self {
        Self {
            id: auto_incr::next("log_id"),
            opered_at: dt::now_dt(),
            oper,
            level,
            message: strip_ansi_escapes::strip_str(message),
            archive_ids,
            vault_ids,
        }
    }

    /// 单条输出，可用于按照log_id展示日志
    pub fn display(&self) {
        let id = match self.oper.source {
            OperSource::User => self.id.styled_id(),
            OperSource::System => self.id.styled_sys_id(),
            OperSource::Transformed => self.id.styled_trans_id(),
        };

        // 此处恰好也可以用表格来输出
        let cols = vec![Column::left("Prop"), Column::left("Value")];
        let rows = vec![
            kv_row!("Id", id),
            kv_row!("Opered At", dt::to_dt_string(&self.opered_at).grey()),
            kv_row!("Level", self.level.to_display()),
            kv_row!("Operation", self.oper.to_detailed_display()),
            kv_row!("Archive Ids", join_archive_ids(&self.archive_ids)),
            kv_row!("Vaults", join_vault_ids(&self.vault_ids)),
            kv_row!("remark", self.message.replace("\n", "\\n").styled_string()),
        ];
        let table = Table::new(cols, rows);

        println!("--- Log:");
        table.display_rows();

        let display_related_list_entries = |ids: &[u32]| {
            let arr = must_ok!(
                archive::list::find(|entry| ids.contains(&entry.id)),
                format!("Cannot find list entries with log id: {}", self.id)
            );

            if arr.len() > 0 {
                println!("--- Related Archiver List Entries:");
                arr.iter().for_each(|entry| {
                    entry.display();
                    println!("---")
                });
            }
        };

        let display_related_vaults = |ids: &[u32]| {
            let arr = vault::find(|entry| ids.contains(&entry.id));

            if arr.len() > 0 {
                println!("--- Related Vaults:");
                arr.iter().for_each(|entry| {
                    entry.display();
                    println!("---")
                });
            }
        };

        // 如果查询的是put、restore记录，那么关联查询list
        if matches!(self.level, LogLevel::Success) {
            match self.oper.main.as_str() {
                main::PUT | main::RESTORE => {
                    if let Some(ids) = &self.archive_ids {
                        display_related_list_entries(ids);
                    }
                }
                main::VAULT => {
                    if let Some(ids) = &self.vault_ids {
                        display_related_vaults(ids);
                    }
                }
                _ => {}
            }
        }
    }
}

impl TableRowify for LogEntry {
    fn to_table_row(&self) -> crate::misc::console::table::TableRow {
        let id = match self.oper.source {
            OperSource::User => self.id.styled_id(),
            OperSource::System => self.id.styled_sys_id(),
            OperSource::Transformed => self.id.styled_trans_id(),
        };

        let mut cells = vec![
            dt::to_omitted_dt_string(&self.opered_at).grey(),
            id,
            self.level.to_mark(),
            self.oper.to_display(),
        ];

        let archive_id = join_archive_ids(&self.archive_ids);
        let vault_name = match self.oper.main.as_str() {
            main::PUT | main::VAULT => join_vault_ids(&self.vault_ids),
            _ => String::new(),
        };

        let message = config::alias::apply(&self.message)
            .replace("\n", "\\n")
            .to_string();

        let avid = match (archive_id.is_empty(), vault_name.is_empty()) {
            (true, true) => String::new(),
            (false, true) => format!("({}{})", *config::VLT_ITEM_SEP, archive_id.styled_id(),),
            (true, false) => format!("({}{})", vault_name.styled_vault(), *config::VLT_ITEM_SEP,),
            (false, false) => format!(
                "({}{}{})",
                vault_name.styled_vault(),
                *config::VLT_ITEM_SEP,
                archive_id.styled_id(),
            ),
        };

        // 下面处理remark、archive_id和vault_name的显示
        let mav = match (self.message.is_empty(), avid.is_empty()) {
            (true, true) => String::new(),
            (false, true) => message.grey(),
            (true, false) => avid,
            (false, false) => format!("{} {}", message.grey(), avid),
        };

        cells.push(mav);

        TableRow::new(cells)
    }

    fn get_table_columns() -> Vec<Column> {
        vec![
            Column::left("Archived At"),
            Column::left("Id"),
            Column::left("⚑"),
            Column::left_nsigma("Oper"),
            Column::left_flex_with_max("Remark", 36),
        ]
    }
}

fn join_archive_ids(ids: &Option<Vec<u32>>) -> String {
    if let Some(ids) = ids {
        ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ")
            .styled_id()
    } else {
        String::new()
    }
}

fn join_vault_ids(ids: &Option<Vec<u32>>) -> String {
    if let Some(ids) = ids {
        ids.iter()
            .map(|n| format!("{}({})", vault::get_name(*n), n.styled_vault()))
            .collect::<Vec<String>>()
            .join(", ")
    } else {
        String::new()
    }
}
