use crate::must_ok;

use super::sl;

/// 分配即持久化
pub fn next() -> u32 {
    let mut auto_incr = must_ok!(sl::load(), "Failed to parse auto increment file");
    auto_incr.vault_id += 1;
    must_ok!(sl::save(&auto_incr), "Failed to update auto increment file");
    auto_incr.vault_id
}
