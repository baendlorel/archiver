use crate::{err_warn, wrap_err_fatal, wrap_result};

use crate::models::error::ArchiverError;

use super::data;

pub fn toggle(status: &str) -> Result<(), ArchiverError> {
    if status != "on" && status != "off" {
        return err_warn!(
            "Status of auto check update must be 'on' or 'off', but got '{}'",
            status
        );
    }

    let mut config = wrap_err_fatal!(data::load())?;
    config.auto_check_update = status.to_string();
    wrap_result!(data::save(&config))?;

    Ok(())
}
