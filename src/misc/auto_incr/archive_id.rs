use crate::uoe_result;

use super::sl;

/// 分配即持久化
pub fn next() -> u32 {
    let mut auto_incr = uoe_result!(sl::load(), "Failed to parse auto increment file");
    auto_incr.archive_id += 1;
    uoe_result!(sl::save(&auto_incr), "Failed to update auto increment file");
    auto_incr.archive_id
}
