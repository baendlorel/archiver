use crate::{warn, wrap_result};

use chrono::Datelike;

use super::{CONFIG, sl};
use crate::misc::dt;
use crate::models::{error::ArchiverResult, types::ArchiverConfig};

pub fn set(status: &str) -> ArchiverResult<()> {
    if status != "on" && status != "off" {
        return warn!(
            "Status of auto check update must be 'on' or 'off', but got '{}'",
            status
        );
    }

    let mut config = CONFIG.clone();
    config.update_check = status.to_string();
    wrap_result!(sl::save(&config))?;

    Ok(())
}

/// 超过特定时间再检查更新
pub fn time_passed(config: &ArchiverConfig) -> bool {
    let today = dt::now_d();
    let last = &config.last_update_check;

    let months_passed =
        (today.year() - last.year()) * 12 + (today.month() as i32 - last.month() as i32);

    // 判断是否超过1个月
    months_passed > 1
}

pub fn refresh_last_date() -> ArchiverResult<()> {
    let mut config = CONFIG.clone();
    config.last_update_check = dt::now_d();
    wrap_result!(sl::save(&config))?;
    Ok(())
}
