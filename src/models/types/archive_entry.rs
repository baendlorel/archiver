use serde::{Deserialize, Serialize};

/// 定义用于序列化到JSON的日志条目结构
#[derive(Serialize, Deserialize)]
pub struct ArchiveEntry {
    pub id: u32,
    pub target: String,
    pub is_dir: bool,
    pub dir: String,
    pub time: String,
}

impl ArchiveEntry {
    pub fn to_str(&self) -> String {
        format!(
            "{} - {} - {} - {} {}",
            self.id,
            self.oper.to_padded_str(),
            self.status,
            self.arg,
            if let Some(id) = self.id {
                format!("(id: {})", id)
            } else {
                "".to_string()
            }
        )
    }
}
