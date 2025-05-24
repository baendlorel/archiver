use crate::{err_warn, uoe_option, wrap_err_fatal};

use chrono::NaiveDateTime;

use crate::misc::dt;
use crate::models::error::ArchiverError;

pub fn parse_range(range: &Option<String>) -> Result<(u32, u32), ArchiverError> {
    let default_a = u32::MIN;
    let default_b = u32::MAX;

    if range.is_none() {
        return Ok((default_a, default_b));
    }

    let range = range.clone().unwrap();

    // 允许看所有
    if range == "all" || range == "*" || range == "a" {
        return Ok((default_a, default_b));
    }

    let is_parsable = |s: &str| -> bool { s == "*" || s.chars().all(|c| c.is_numeric()) };

    let parse = |s: &str, default_value: u32| -> Result<u32, ArchiverError> {
        if s == "*" {
            return Ok(default_value);
        }

        let is_valid_len = s.len() > 2;
        if !is_valid_len {
            return err_warn!("Length of date string must > 2");
        }

        let raw_month = wrap_err_fatal!(s[(s.len() - 2)..s.len()].parse::<u32>())?;
        if raw_month > 12 || raw_month < 1 {
            return err_warn!("Month must be 1~12. Got '{}'", raw_month);
        }

        Ok(wrap_err_fatal!(s.parse::<u32>())?)
    };

    if is_parsable(&range) {
        return Ok((parse(&range, default_a)?, default_b));
    }

    if let Some((a_str, b_str)) = range.split_once('-') {
        if !is_parsable(&a_str.to_string()) {
            return err_warn!(
                "Start date is not * or contains letters other than digits. Got '{}'",
                a_str
            );
        }
        if !is_parsable(&b_str.to_string()) {
            return err_warn!(
                "End date is not * or contains letters other than digits. Got '{}'",
                b_str
            );
        }

        let a = parse(&a_str.to_string(), default_a)?;
        let b = parse(&b_str.to_string(), default_b)?;

        if a > b {
            return err_warn!("Start date > end date");
        }
        return Ok((a, b));
    }

    err_warn!("Must give args like `202501`, `202501-202506`,`*-202501`")
}

pub fn normalize_range(
    range: &Option<String>,
) -> Result<(NaiveDateTime, NaiveDateTime), ArchiverError> {
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
        let d = dt::parse_from_ym(&range)?;
        // 这里的and_hms_opt不会错的，要错也是上面dt的parse有错
        // 所以使用uoe_option宏
        let dt_a = uoe_option!(d.and_hms_opt(0, 0, 0), "Failed to create start dt");
        let dt_b = uoe_option!(d.and_hms_opt(23, 59, 59), "Failed to create end dt");
        return Ok((dt_a, dt_b));
    }

    // 2. YYYYMM-YYYYMM
    if let Some((a_str, b_str)) = range.split_once('-') {
        if !is_numeric(&a_str) || !is_numeric(&b_str) {
            return err_warn!("Range must be numeric or *");
        }

        let dt_a = if a_str == "*" {
            dt::MIN_DT
        } else {
            let d = dt::parse_from_ym(a_str)?;
            uoe_option!(d.and_hms_opt(0, 0, 0), "Failed to create start dt")
        };

        let dt_b = if b_str == "*" {
            dt::MAX_DT
        } else {
            let d = dt::parse_from_ym(b_str)?;
            uoe_option!(d.and_hms_opt(23, 59, 59), "Failed to create end dt")
        };

        if dt_a > dt_b {
            return err_warn!("Start date > end date");
        }

        return Ok((dt_a, dt_b));
    }

    err_warn!("<range> must be YYYYMM or YYYYMM-YYYYMM")
}
