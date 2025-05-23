use serde::{Deserialize, Serialize};

fn default_vault_id() -> u32 {
    0
}

fn default_archive_id() -> u32 {
    0
}

#[derive(Serialize, Deserialize)]
pub struct AutoIncr {
    #[serde(default = "default_vault_id")]
    /// 库的id，会自增
    pub vault_id: u32,

    #[serde(default = "default_archive_id")]
    /// 自动检查更新的开关，默认为on
    pub archive_id: u32,
}

impl AutoIncr {
    pub fn default() -> AutoIncr {
        Self {
            vault_id: 0,
            archive_id: 0,
        }
    }
}
