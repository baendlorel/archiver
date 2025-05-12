use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use super::field_style;

#[derive(Serialize, Deserialize)]
pub struct ListEntry {
    pub id: u32,
    pub target: String,
    pub is_dir: bool,
    pub dir: String,
    pub time: String,
}

impl ListEntry {
    pub fn to_str(&self) -> String {
        format!(
            "{} {} - {} - {} - {}",
            field_style::grey(&self.time),
            field_style::id_to_str(self.id),
            if self.is_dir {
                "dir ".cyan().to_string()
            } else {
                "file".yellow().to_string()
            },
            self.target,
            field_style::cwd(&self.dir),
        )
    }
}
