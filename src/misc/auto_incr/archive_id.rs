use crate::uoe_result;

use super::sl;

pub fn next() -> u32 {
    let auto_incr = uoe_result!(sl::load(), "Failed to parse auto increment file");
    auto_incr.archive_id + 1
}

pub fn update(next_id: u32) {
    let mut auto_incr = uoe_result!(sl::load(), "Failed to parse auto increment file");
    auto_incr.archive_id = next_id;

    uoe_result!(sl::save(&auto_incr), "Failed to update auto increment file");
}
