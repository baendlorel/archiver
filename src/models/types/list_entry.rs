use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use super::field_style::Grey;
use crate::misc::paths;

#[derive(Serialize, Deserialize)]
pub struct ListEntry {
    /// 归档ID，后续使用这个来restore
    pub id: u32,

    /// 归档目标名，可能是文件名或文件夹名
    pub target: String,

    /// 是否已经恢复
    pub is_restored: bool,

    /// 归档目标是否为文件夹
    pub is_dir: bool,

    /// 归档目标的原始路径
    pub dir: String,

    /// 归档时间
    pub time: String,
}

/// 专门输出表格用的
pub struct ListRow {
    pub time: String,

    pub id: String,

    pub target: String,

    pub dir: String,

    pub _width: ListRowColWidth,
}

pub struct ListRowColWidth {
    pub time: usize,
    pub id: usize,
    pub target: usize,
    pub dir: usize,
}

impl ListEntry {
    pub fn get_target_path(&self) -> String {
        let target_name = &self.target;
        let dir = paths::apply_alias(&self.dir);
        format!("{}{}{}", dir, std::path::MAIN_SEPARATOR, target_name)
    }

    pub fn to_row(&self) -> ListRow {
        let is_restored = if self.is_restored {
            "(R)".fg_rgb::<255, 165, 0>().to_string()
        } else {
            "".to_string()
        };

        let target = if self.is_dir {
            format!("{}{}", self.target.blue(), std::path::MAIN_SEPARATOR)
        } else {
            self.target.to_string()
        };

        let dir = paths::apply_alias(&self.dir);

        let get_target_width = || -> usize {
            let base = self.target.len();
            let rst = if self.is_restored { 3 } else { 0 };
            let dirslash = if self.is_dir { 1 } else { 0 };
            base + rst + dirslash
        };

        ListRow {
            time: self.time.grey(),
            id: self.id.magenta().to_string(),
            // id: field_style::id_to_str(self.id),
            target: target + &is_restored,
            dir: dir.grey(),
            _width: ListRowColWidth {
                time: self.time.len(),
                id: self.id.to_string().len(),
                target: get_target_width(),
                dir: dir.len(),
            },
        }
    }
}
