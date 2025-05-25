use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{core::auto_incr, misc::dt, models::serde_custom::naive_date_time};

#[derive(Serialize, Deserialize, Clone)]
pub struct Vault {
    /// 库的id，会自增
    pub id: u32,

    /// 库的名字
    #[serde(rename = "n")]
    pub name: String,

    /// 库的备注，可以记录用途
    #[serde(rename = "r")]
    pub remark: String,

    /// 库的创建时间
    #[serde(rename = "cat", with = "naive_date_time")]
    pub created_at: NaiveDateTime,

    /// 库状态
    #[serde(rename = "st")]
    pub status: VaultStatus,
}

#[derive(Serialize, Deserialize, Clone)]
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
            remark: String::new(),
            created_at: dt::start_dt(),
            status: VaultStatus::Valid,
        }
    }

    pub fn new(name: &str, remark: Option<String>) -> Self {
        Vault {
            id: auto_incr::vault_id::next(),
            name: name.to_string(),
            remark: remark.unwrap_or(String::new()),
            created_at: dt::now_dt(),
            status: VaultStatus::Valid,
        }
    }
}
