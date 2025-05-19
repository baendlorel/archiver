use crate::{err_warn, wrap_err, wrap_result};

use crate::models::error::ArchiverError;

use super::config_data;

pub fn toggle(status: &str) -> Result<(), ArchiverError> {
    if status != "on" && status != "off" {
        return Err(err_warn!(format!(
            "Status of auto check update must be 'on' or 'off', but got '{}'",
            status
        )));
    }

    let mut config = wrap_err!(config_data::load())?;
    config.auto_check_update = status.to_string();
    wrap_result!(config_data::save(&config))?;

    Ok(())
}
