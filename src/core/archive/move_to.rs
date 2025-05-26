use crate::as_fatal;

use std::collections::HashSet;
use std::fs;

use super::sl;
use crate::core::vault;
use crate::models::error::ArchiverError;
use crate::models::types::ListEntry;

// todo 所有多重输入，为了反复根据id查询，可能应该把sl::load改为返回HashMap<u32, ListEntry>，同时配套save入参
pub fn do_the_move(entry: &ListEntry, vault_id: u32) -> Result<(), ArchiverError> {
    // as_fatal!(fs::rename(from, to));

    Ok(())
}
