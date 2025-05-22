use crate::{err_warn, wrap_err};

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

        let raw_month = wrap_err!(s[(s.len() - 2)..s.len()].parse::<u32>())?;
        if raw_month > 12 || raw_month < 1 {
            return err_warn!("Month must be 1~12. Got '{}'", raw_month);
        }

        Ok(wrap_err!(s.parse::<u32>())?)
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
