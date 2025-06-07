use crate::kv_row;

use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::core::{auto_incr, config, vault};
use crate::misc::console::table::{Column, Table, TableRow, TableRowify};
use crate::misc::dt;
use crate::models::serde_custom::{boolean, naive_date_time};
use crate::traits::CustomColors;

/// 归档列表的条目
/// - 这样的字段排序是为了序列化的时候jsonl文件也可以是这个顺序
#[derive(Serialize, Deserialize, Clone)]
pub struct ListEntry {
    /// 归档时间
    #[serde(rename = "aat", with = "naive_date_time")]
    pub archived_at: NaiveDateTime,

    /// 是否已经恢复
    #[serde(rename = "st")]
    pub status: ListStatus,

    /// 归档目标是否为文件夹
    #[serde(rename = "is_d", with = "boolean")]
    pub is_dir: bool,

    /// 库id
    #[serde(rename = "vid")]
    pub vault_id: u32,

    /// 归档ID，后续使用这个来restore
    pub id: u32,

    /// 归档目标名，可能是文件名或文件夹名
    #[serde(rename = "i")]
    pub item: String,

    /// 归档目标的原始路径
    #[serde(rename = "d")]
    pub dir: String,

    /// 信息，可以写为什么归档
    #[serde(rename = "m")]
    pub message: String,

    /// 备注
    #[serde(rename = "r")]
    pub remark: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ListStatus {
    /// 已归档
    #[serde(rename = "A")]
    Archived,

    /// 已恢复
    #[serde(rename = "R")]
    Restored,
}

impl ListEntry {
    pub fn new(item: String, is_dir: bool, dir: String, message: String, vault_id: u32) -> Self {
        Self {
            // 这些是传入参数
            item,
            is_dir,
            dir,
            message,
            vault_id,

            // 这些是自动获取
            id: auto_incr::next("archive_id"),
            archived_at: dt::now_dt(),
            status: ListStatus::Archived,
            remark: String::new(),
        }
    }

    pub fn get_item_path_string(&self) -> String {
        let dir = config::alias::apply(&self.dir);
        format!("{}{}{}", dir, std::path::MAIN_SEPARATOR, &self.item)
    }

    pub fn is_restored(&self) -> bool {
        matches!(self.status, ListStatus::Restored)
    }

    /// 用于单条记录详细信息的输出
    pub fn display(&self) {
        let item = if self.is_dir {
            format!("{}{}", self.item.styled_dir(), std::path::MAIN_SEPARATOR)
        } else {
            self.item.clone()
        };

        let archived_at = dt::to_dt_string(&self.archived_at).grey();
        let dir = config::alias::apply(&self.dir).bright_grey();
        let vault_info =
            format!("{}({})", vault::get_name(self.vault_id), self.vault_id).styled_vault();

        // 此处恰好也可以用表格来输出
        let cols = vec![Column::left("Prop"), Column::left("Value")];
        let rows = vec![
            kv_row!("Archive Id", self.id.styled_id()),
            kv_row!("Vault", vault_info),
            kv_row!("Archived At", archived_at),
            kv_row!("Source Dir", dir),
            kv_row!("Item Name", item),
            kv_row!("Status", self.status.to_display()),
        ];
        Table::new(cols, rows).display_rows();
    }
}

impl TableRowify for ListEntry {
    fn to_table_row(&self) -> TableRow {
        let item = {
            let t = if self.is_dir {
                format!("{}{}", self.item.styled_dir(), std::path::MAIN_SEPARATOR)
            } else {
                self.item.clone()
            };

            let r = match self.status {
                ListStatus::Restored => "(R)".orange(),
                _ => String::new(),
            };

            let v = vault::get_name(self.vault_id).styled_vault();
            format!("{}{}{}{}", v, *config::VLT_ITEM_SEP, t, r)
        };

        let archived_at = dt::to_omitted_dt_string(&self.archived_at).grey();
        let id = self.id.styled_id();
        let dir = config::alias::apply(&self.dir).bright_grey();
        TableRow::new(vec![archived_at, id, item, dir])
    }

    fn get_table_columns() -> Vec<Column> {
        vec![
            Column::left("Archived At"),
            Column::left("Id"),
            Column::left_with_max("Item", 30),
            Column::left_flex("Directory"),
        ]
    }
}

impl ListStatus {
    pub fn to_display(&self) -> String {
        match self {
            ListStatus::Archived => "Archived".bright_blue().to_string(),
            ListStatus::Restored => "Restored".orange(),
        }
    }
}
