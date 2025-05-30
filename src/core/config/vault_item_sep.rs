use crate::wrap_result;

use super::{CONFIG, sl};
use crate::models::error::ArchiverResult;

pub fn set(sep: &str) -> ArchiverResult<()> {
    let mut config = CONFIG.clone();
    config.vault_item_sep = sep.to_string();
    wrap_result!(sl::save(&config))?;
    Ok(())
}
