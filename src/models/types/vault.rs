use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::core::auto_incr;
use crate::misc::console::table::{Column, ColumnAlign, TableRow, TableRowify};
use crate::misc::dt;
use crate::models::serde_custom::naive_date_time;
use crate::traits::CustomColors;

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

impl TableRowify for Vault {
    fn to_table_row(&self) -> TableRow {
        let cells = vec![
            dt::to_dt_string(&self.created_at)
                .bright_black()
                .to_string(),
            self.id.styled_vault(),
            self.name.clone(),
            match self.status {
                VaultStatus::Valid => "Valid".styled_valid(),
                VaultStatus::Removed => "Removed".styled_invalid(),
                VaultStatus::Protected => "Protected".styled_const(),
            },
            self.remark.bright_black().to_string(),
        ];
        TableRow::new(cells)
    }

    fn get_table_columns() -> Vec<Column> {
        vec![
            Column::left("Created At"),
            Column::left("Id"),
            Column::left("Name"),
            Column::left("Status"),
            Column::new("Remark", ColumnAlign::Left, (6, 25)),
        ]
    }
}
