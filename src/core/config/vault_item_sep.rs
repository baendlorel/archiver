use crate::{warn, wrap_result};

use super::{CONFIG, sl};
use crate::models::error::ArchiverResult;

pub fn set(sep: &str) -> ArchiverResult<()> {
    if sep.is_empty() {
        return warn!("Vault item separator cannot be empty.");
    }

    let mut config = CONFIG.clone();
    config.vault_item_sep = sep.to_string();
    wrap_result!(sl::save(&config))?;
    Ok(())
}
