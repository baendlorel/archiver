use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use super::field_style;
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

pub struct ListRowField {
    pub time: String,
    pub id: String,
    pub target: String,
    pub dir: String,
}

pub static LIST_ROW_FIELD: Lazy<ListRowField> = Lazy::new(|| ListRowField {
    time: "Archived At".to_string(),
    id: "ID".to_string(),
    target: "Item".to_string(),
    dir: "Directory".to_string(),
});

impl ListEntry {
    pub fn to_row(&self) -> ListRow {
        let is_restored = if self.is_restored {
            "(R)".fg_rgb::<255, 165, 0>().to_string()
        } else {
            "".to_string()
        };

        let dir = paths::apply_alias(&self.dir);

        ListRow {
            time: field_style::grey(&self.time),
            id: self.id.magenta().to_string(),
            // id: field_style::id_to_str(self.id),
            target: field_style::target_color(&self.target, self.is_dir) + &is_restored,
            dir: field_style::grey(&dir),
            _width: ListRowColWidth {
                time: self.time.len(),
                id: self.id.to_string().len(),
                target: self.target.len() + if self.is_restored { 3 } else { 0 },
                dir: dir.len(),
            },
        }
    }
}
