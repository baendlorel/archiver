use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::core::{auto_incr, vault};
use crate::misc::{dt, paths};
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
    #[serde(rename = "t")]
    pub target: String,

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
    pub vault_name: String,
    pub id: String,
    pub target: String,
    pub dir: String,
}

pub struct ListColumnLen {
    pub archived_at: usize,
    pub vault_name: usize,
    pub id: usize,
    pub target: usize,
    pub dir: usize,
}

impl ListEntry {
    pub fn new(target: String, is_dir: bool, dir: String, message: String, vault_id: u32) -> Self {
        Self {
            // 这些是传入参数
            target,
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

    pub fn get_target_path_string(&self) -> String {
        let target_name = &self.target;
        let dir = paths::apply_alias(&self.dir);
        format!("{}{}{}", dir, std::path::MAIN_SEPARATOR, target_name)
    }

    pub fn to_row(&self) -> ListRow {
        let target = {
            let t = if self.is_dir {
                format!("{}{}", self.target.styled_dir(), std::path::MAIN_SEPARATOR)
            } else {
                self.target.clone()
            };

            let r = match self.status {
                ListStatus::Archived => String::new(),
                ListStatus::Restored => "(R)".orange(),
            };

            format!("{}{}", t, r)
        };

        ListRow {
            archived_at: dt::to_dt_string(&self.archived_at)
                .bright_black()
                .to_string(),
            id: self.id.styled_archive_id(),
            vault_name: vault::get_name(self.vault_id).styled_vault(),
            target,
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
            archived_at: self.archived_at.len(),
            vault_name: self.vault_name.len(),
            id: self.id.len(),
            target: self.target.len(),
            dir: self.dir.len(),
        }
    }

    pub fn to_display(&self, max_len: &ListColumnLen) -> String {
        let cl = self.get_len();
        format!(
            "{}{} {}{} {}{} {}{} {}",
            self.archived_at.bright_black(),
            " ".repeat(max_len.archived_at - cl.archived_at),
            self.vault_name.styled_vault(),
            " ".repeat(max_len.vault_name - cl.vault_name),
            self.id.styled_archive_id(),
            " ".repeat(max_len.id - cl.id),
            self.target,
            " ".repeat(max_len.target - cl.target),
            self.dir.bright_black(),
        )
    }
}
