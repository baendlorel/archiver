use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::field_style;

#[derive(Serialize, Deserialize, Tabled)]
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
#[derive(Tabled)]
pub struct ListRow {
    #[tabled(rename = "Archived at")]
    pub time: String,

    #[tabled(rename = "ID")]
    pub id: String,

    #[tabled(rename = "Target")]
    pub target: String,

    #[tabled(rename = "Origin Dir")]
    pub dir: String,
}

impl ListEntry {
    pub fn to_log(&self) -> String {
        let is_restored = if self.is_restored {
            "(R)".fg_rgb::<255, 165, 0>().to_string()
        } else {
            "".to_string()
        };

        format!(
            "{} {} - {} {} - {}",
            field_style::grey(&self.time),
            field_style::id_to_str(self.id),
            field_style::dir_color(&self.target, self.is_dir),
            is_restored,
            field_style::cwd(&self.dir),
        )
    }

    pub fn to_row(&self) -> ListRow {
        let is_restored = if self.is_restored {
            "(R)".fg_rgb::<255, 165, 0>().to_string()
        } else {
            "".to_string()
        };

        ListRow {
            time: field_style::grey(&self.time),
            id: self.id.magenta().to_string(),
            target: field_style::dir_color(&self.target, self.is_dir) + &is_restored,
            dir: field_style::cwd(&self.dir),
        }
    }
}
