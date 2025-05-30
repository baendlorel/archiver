// 模块结构
mod common;
mod sl;

pub mod alias;
pub mod update_check;
pub mod vault_item_sep;
pub use common::{CONFIG, VLT_ITEM_SEP};
pub use sl::save;

pub fn display() {
    CONFIG.display();
}
