use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vault {
    /// 库的id，会自增
    pub id: u32,

    /// 库的名字
    pub name: String,

    /// 库的备注，可以记录用途
    pub remark: String,

    /// 库的创建时间
    pub created_at: String,
}
