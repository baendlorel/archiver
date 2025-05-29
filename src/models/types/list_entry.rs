use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::path::MAIN_SEPARATOR;

use crate::core::{auto_incr, vault};
use crate::misc::{dt, paths};
use crate::models::serde_custom::{boolean, naive_date_time};
use crate::traits::CustomColors;
use crate::traits::strip_ansi::StripAnsi;

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

/// 专门输出表格用的
pub struct ListRow {
    pub archived_at: String,
    pub id: String,
    pub item: String,
    pub dir: String,
}

pub struct ListColumnLen {
    pub archived_at: usize,
    pub id: usize,
    pub item: usize,
    pub dir: usize,
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
            id: auto_incr::archive_id::next(),
            archived_at: dt::now_dt(),
            status: ListStatus::Archived,
            remark: String::new(),
        }
    }

    pub fn get_item_path_string(&self) -> String {
        let dir = paths::apply_alias(&self.dir);
        format!("{}{}{}", dir, std::path::MAIN_SEPARATOR, &self.item)
    }

    pub fn to_row(&self) -> ListRow {
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
            format!("{}{}{}{}", v, MAIN_SEPARATOR, t, r)
        };

        ListRow {
            archived_at: dt::to_dt_string(&self.archived_at)
                .bright_black()
                .to_string(),
            id: self.id.styled_archive_id(),
            item,
            dir: paths::apply_alias(&self.dir).bright_grey(),
        }
    }

    pub fn is_restored(&self) -> bool {
        matches!(self.status, ListStatus::Restored)
    }
}

impl ListRow {
    pub fn get_len(&self) -> ListColumnLen {
        ListColumnLen {
            archived_at: self.archived_at.true_len(),
            id: self.id.true_len(),
            item: self.item.true_len(),
            dir: self.dir.true_len(),
        }
    }

    pub fn to_display(&self, max_len: &ListColumnLen) -> String {
        let cl = self.get_len();
        format!(
            "{}{} {}{} {}{} {}",
            self.archived_at.bright_black(),
            " ".repeat(max_len.archived_at - cl.archived_at),
            self.id.styled_archive_id(),
            " ".repeat(max_len.id - cl.id),
            self.item,
            " ".repeat(max_len.item - cl.item),
            self.dir.bright_black(),
        )
    }
}
