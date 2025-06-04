use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{core::auto_incr, misc::dt, models::serde_custom::naive_date_time};

pub mod vault_defaults {
    /// 默认仓库的名字，确定了以后更新就不太好改了
    /// - 此符号是Copilot、deepseek、GPT三位AI都推荐的
    ///     - 它是从这些符号中胜出的`-`，`+`，`#`，`$`，`=`，`@`
    pub const NAME: &str = "@";
    pub const ID: u32 = 0;
}

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
    Protected,
}

impl Vault {
    pub fn default() -> Self {
        Vault {
            id: 0,
            name: vault_defaults::NAME.to_string(),
            remark: String::new(),
            created_at: dt::start_dt(),
            status: VaultStatus::Valid,
        }
    }

    pub fn new(name: &str, remark: Option<String>) -> Self {
        Vault {
            id: auto_incr::next("vault_id"),
            name: name.to_string(),
            remark: remark.unwrap_or(String::new()),
            created_at: dt::now_dt(),
            status: VaultStatus::Valid,
        }
    }
}
