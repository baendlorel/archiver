use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ArchiverConfig {
    pub auto_check_update: String,
    /// 别名映射表
    pub alias_list: Vec<AliasEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct AliasEntry {
    pub alias: String,
    pub origin: String,
}
