use crate::{must_some, warn};

use chrono::NaiveDateTime;

use crate::misc::dt;
use crate::models::error::ArchiverResult;

pub fn normalize_range(range: &Option<String>) -> ArchiverResult<(NaiveDateTime, NaiveDateTime)> {
    let default_a = dt::MIN_DT;
    let default_b = dt::MAX_DT;

    if range.is_none() {
        return Ok((default_a, default_b));
    }

    let range = range.clone().unwrap();
    if range == "all" || range == "*" || range == "a" {
        return Ok((default_a, default_b));
    }

    // 接下来有YYYYMM和YYYYMM-YYYYMM两种情况
    // 1. YYYYMM
    let is_numeric = |s: &str| -> bool { s.chars().all(|c| c.is_numeric()) };
    if is_numeric(&range) {
        let d_a = dt::parse_compact_ymd(&format!("{}01", range))?;
        let d_b = dt::to_last_date(&d_a)?;

        // 这里and_hms_opt不会错，要也是dt的parse有错,所以使用uoe_option宏
        let dt_a = must_some!(d_a.and_hms_opt(0, 0, 0), "Failed to get dt_a");
        let dt_b = must_some!(d_b.and_hms_opt(23, 59, 59), "Failed to get dt_b");
        return Ok((dt_a, dt_b));
    }

    // 2. YYYYMM-YYYYMM
    if let Some((a_str, b_str)) = range.split_once('-') {
        if !is_numeric(&a_str) || !is_numeric(&b_str) {
            return warn!("Range must be numeric or *");
        }

        let dt_a = if a_str == "*" {
            dt::MIN_DT
        } else {
            let d = dt::parse_compact_ymd(&format!("{}01", a_str))?;
            // 这里and_hms_opt不会错，要也是dt的parse有错,所以使用uoe_option宏
            must_some!(d.and_hms_opt(0, 0, 0), "Failed to create start dt")
        };

        let dt_b = if b_str == "*" {
            dt::MAX_DT
        } else {
            let d = dt::parse_compact_ymd(&format!("{}01", b_str))?;
            let dt = dt::to_last_date(&d)?;
            // 这里and_hms_opt不会错，要也是dt的parse有错,所以使用uoe_option宏
            must_some!(dt.and_hms_opt(23, 59, 59), "Failed to create end dt")
        };

        if dt_a > dt_b {
            return warn!("Start date > end date");
        }

        return Ok((dt_a, dt_b));
    }

    warn!("<range> must be YYYYMM or YYYYMM-YYYYMM")
}
