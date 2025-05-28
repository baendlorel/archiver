use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AutoIncrVars {
    /// 库的id，会自增
    pub vault_id: u32,

    /// 自动检查更新的开关，默认为on
    pub archive_id: u32,
}

impl AutoIncrVars {
    pub fn default() -> AutoIncrVars {
        Self {
            vault_id: 0,
            archive_id: 0,
        }
    }
}
