use crate::uoe_result;

use std::fs;

use super::paths;

pub fn next_id() -> u32 {
    let auto_incr_file = paths::AUTO_INCR_FILE_PATH.clone();
    if !auto_incr_file.exists() {
        uoe_result!(
            fs::write(&auto_incr_file, "1"),
            "Failed to create auto increment file"
        );
        return 1;
    }
    let content = uoe_result!(
        fs::read_to_string(&auto_incr_file),
        "Failed to read auto increment file"
    );

    let current_id = uoe_result!(
        content.trim().parse::<u32>(),
        "Failed to parse auto increment value"
    );

    1 + current_id
}

pub fn update_id(new_id: u32) {
    let auto_incr_file = paths::AUTO_INCR_FILE_PATH.clone();
    uoe_result!(
        fs::write(&auto_incr_file, new_id.to_string()),
        "Failed to update auto increment file"
    );
}
