use crate::{warn, wrap_result};

use std::collections::HashSet;

use super::sl;
use crate::core::vault;
use crate::models::error::ArchiverError;

pub fn move_to(ids: &[u32], to: &str) -> Result<(usize, usize), ArchiverError> {
    let vault = match vault::find_by_name(to) {
        Some(vault) => vault,
        None => {
            return warn!("Vault '{}' not found", to);
        }
    };

    let mut full_list = wrap_result!(sl::load())?;
    let mut count: usize = 0;

    // todo 所有多重输入的都需要用set去重
    let id_set: HashSet<u32> = ids.iter().cloned().collect();

    full_list.iter_mut().for_each(|entry| {
        if id_set.contains(&entry.id) && entry.vault_id != vault.id {
            entry.vault_id = vault.id;
            count += 1;
        }
    });

    Ok((count, id_set.len()))
}
