use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{core::auto_incr, misc::dt};

#[derive(Serialize, Deserialize)]
pub struct Vault {
    /// 库的id，会自增
    pub id: u32,

    /// 库的名字
    pub name: String,

    /// 库的备注，可以记录用途
    pub remark: String,

    /// 库的创建时间
    pub created_at: NaiveDateTime,

    /// 库状态
    pub status: VaultStatus,
}

#[derive(Serialize, Deserialize)]
pub enum VaultStatus {
    Valid,
    Removed,
    Hidden,
}

impl Vault {
    pub fn default() -> Self {
        Vault {
            id: 0,
            name: "default".to_string(),
            remark: "".to_string(),
            created_at: dt::start_dt(),
            status: VaultStatus::Valid,
        }
    }

    pub fn new(name: &str, remark: Option<String>) -> Self {
        Vault {
            id: auto_incr::vault_id::next(),
            name: name.to_string(),
            remark: remark.unwrap_or("".to_string()),
            created_at: dt::now_dt(),
            status: VaultStatus::Valid,
        }
    }
}
