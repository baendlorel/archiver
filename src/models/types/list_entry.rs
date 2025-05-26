use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use super::CONFIG;
use crate::core::{auto_incr, vault};
use crate::misc::{CustomColors, dt, paths};
use crate::models::serde_custom::{boolean, naive_date_time};

/// 归档列表的条目
/// - 这样的字段排序是为了序列化的时候jsonl文件也可以是这个顺序
#[derive(Serialize, Deserialize, Clone)]
pub struct ListEntry {
    /// 归档时间
    #[serde(rename = "aat", with = "naive_date_time")]
    pub archived_at: NaiveDateTime,

    /// 是否已经恢复
    #[serde(rename = "is_r", with = "boolean")]
    pub is_restored: bool,

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

/// 专门输出表格用的
pub struct ListRow {
    pub archived_at: String,
    pub vault_name: String,
    pub id: String,
    pub target: String,
    pub is_restored: bool,
    pub is_dir: bool,
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
    pub fn new(target: String, is_dir: bool, dir: String, message: String) -> Self {
        Self {
            // 这些是传入参数
            target,
            is_dir,
            dir,
            message,

            // 这些是自动获取
            id: auto_incr::archive_id::next(),
            vault_id: CONFIG.current_vault_id,
            archived_at: dt::now_dt(),
            is_restored: false,
            remark: String::new(),
        }
    }

    pub fn get_target_path_string(&self) -> String {
        let target_name = &self.target;
        let dir = paths::apply_alias(&self.dir);
        format!("{}{}{}", dir, std::path::MAIN_SEPARATOR, target_name)
    }

    pub fn to_row(&self) -> ListRow {
        ListRow {
            archived_at: dt::to_dt_string(&self.archived_at),
            id: self.id.to_string(),
            vault_name: vault::get_name(self.vault_id),
            target: self.target.to_string(),
            is_dir: self.is_dir,
            is_restored: self.is_restored,
            dir: paths::apply_alias(&self.dir),
        }
    }
}

impl ListRow {
    pub fn get_len(&self) -> ListColumnLen {
        let target_len = {
            let base = self.target.len();
            let rst = if self.is_restored { 3 } else { 0 };
            let slash = if self.is_dir { 1 } else { 0 };
            base + rst + slash
        };

        ListColumnLen {
            archived_at: self.archived_at.len(),
            vault_name: self.vault_name.len(),
            id: self.id.len(),
            target: target_len,
            dir: self.dir.len(),
        }
    }

    pub fn to_display(&self, max_len: &ListColumnLen) -> String {
        let cl = self.get_len();

        let target = format!(
            "{}{}{}",
            if self.is_dir {
                self.target.colored_dir().to_string()
            } else {
                self.target.to_string()
            },
            if self.is_restored {
                "(R)".orange()
            } else {
                String::new()
            },
            if self.is_dir {
                std::path::MAIN_SEPARATOR.to_string()
            } else {
                String::new()
            }
        );

        format!(
            "{}{} {}{} {}{} {}{} {}",
            self.archived_at.bright_black(),
            " ".repeat(max_len.archived_at - cl.archived_at),
            self.vault_name.colored_vault(),
            " ".repeat(max_len.vault_name - cl.vault_name),
            self.id.colored_archive_id(),
            " ".repeat(max_len.id - cl.id),
            target,
            " ".repeat(max_len.target - cl.target),
            self.dir.bright_black(),
        )
    }
}
