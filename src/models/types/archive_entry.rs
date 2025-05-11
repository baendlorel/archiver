use serde::{Deserialize, Serialize};

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
            "[{}] - {} - {} - {} - {}",
            self.id,
            self.target,
            if self.is_dir { "dir " } else { "file" },
            self.dir,
            self.time,
        )
    }
}
