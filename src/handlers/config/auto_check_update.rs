use crate::{err_warn, wrap_err_fatal, wrap_result};

use chrono::Datelike;

use super::sl;
use crate::{
    misc::dt,
    models::{error::ArchiverError, types::ArchiverConfig},
};

pub fn toggle(status: &str) -> Result<(), ArchiverError> {
    if status != "on" && status != "off" {
        return err_warn!(
            "Status of auto check update must be 'on' or 'off', but got '{}'",
            status
        );
    }

    let mut config = wrap_err_fatal!(sl::load())?;
    config.auto_check_update = status.to_string();
    wrap_result!(sl::save(&config))?;

    Ok(())
}

/// 超过特定时间再检查更新
pub fn overdue(config: &ArchiverConfig) -> bool {
    let today = dt::now_d();
    let last = &config.last_check_update_date;

    let months_passed =
        (today.year() - last.year()) * 12 + (today.month() as i32 - last.month() as i32);

    // 判断是否超过1个月
    months_passed > 1
}

pub fn refresh(config: &mut ArchiverConfig) -> Result<(), ArchiverError> {
    config.last_check_update_date = dt::now_d();
    wrap_result!(sl::save(&config))?;
    Ok(())
}
