use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use super::{CONFIG, field_style::CustomColors};
use crate::{
    core::{auto_incr, vault},
    misc::{dt, paths},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ListEntry {
    /// 归档ID，后续使用这个来restore
    pub id: u32,

    /// 库id
    pub vault_id: u32,

    /// 归档目标名，可能是文件名或文件夹名
    pub target: String,

    /// 是否已经恢复
    pub is_restored: bool,

    /// 归档目标是否为文件夹
    pub is_dir: bool,

    /// 归档目标的原始路径
    pub dir: String,

    /// 归档时间
    pub time: NaiveDateTime,
}

/// 专门输出表格用的
/// todo 改成全部字段，然后to_styled方法，入参是字段间隔，回参是上色后的样子
pub struct ListRow {
    pub time: String,
    pub vault_name: String,
    pub id: String,
    pub target: String,
    pub is_restored: bool,
    pub is_dir: bool,
    pub dir: String,
}

pub struct ListColumnLen {
    pub time: usize,
    pub vault_name: usize,
    pub id: usize,
    pub target: usize,
    pub dir: usize,
}

impl ListEntry {
    pub fn new(target: String, is_dir: bool, dir: String) -> Self {
        Self {
            id: auto_incr::archive_id::next(),
            vault_id: CONFIG.current_vault_id,
            target,
            is_dir,
            dir,
            time: dt::now_dt(),
            is_restored: false,
        }
    }

    pub fn get_target_path_string(&self) -> String {
        let target_name = &self.target;
        let dir = paths::apply_alias(&self.dir);
        format!("{}{}{}", dir, std::path::MAIN_SEPARATOR, target_name)
    }

    pub fn to_row(&self) -> ListRow {
        let time = dt::to_dt_string(&self.time);

        let target = if self.is_dir {
            format!("{}{}", self.target.blue(), std::path::MAIN_SEPARATOR)
        } else {
            self.target.to_string()
        };

        let dir = paths::apply_alias(&self.dir);

        ListRow {
            time,
            id: self.id.to_string(),
            vault_name: vault::get_name(self.vault_id),
            target,
            is_dir: self.is_dir,
            is_restored: self.is_restored,
            dir,
        }
    }
}

impl ListRow {
    pub fn get_len(&self) -> ListColumnLen {
        let target_len = {
            let base = self.target.len();
            let rst = if self.is_restored { 3 } else { 0 };
            let dirslash = if self.is_dir { 1 } else { 0 };
            base + rst + dirslash
        };

        ListColumnLen {
            time: self.time.len(),
            vault_name: self.vault_name.len(),
            id: self.id.len(),
            target: target_len,
            dir: self.dir.len(),
        }
    }

    pub fn to_styled(&self, max_len: &ListColumnLen) -> String {
        let cl = self.get_len();

        let target = format!(
            "{}{}{}",
            self.target,
            if self.is_restored {
                "(R)".orange()
            } else {
                "".to_string()
            },
            if self.is_dir {
                std::path::MAIN_SEPARATOR.to_string()
            } else {
                "".to_string()
            }
        );

        format!(
            "{}{} {}{} {}{} {}{} {}",
            self.time.grey(),
            " ".repeat(max_len.time - cl.time),
            self.vault_name.bright_blue(),
            " ".repeat(max_len.vault_name - cl.vault_name),
            self.id.magenta(),
            " ".repeat(max_len.id - cl.id),
            target,
            " ".repeat(max_len.target - cl.target),
            self.dir.grey(),
        )
    }
}
