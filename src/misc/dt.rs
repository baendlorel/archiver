use crate::{as_fatal, must_ok, must_some};

use chrono::{Datelike, Local, Months, NaiveDate, NaiveDateTime};

use crate::models::error::ArchiverResult;

// # 日期常量
pub const MIN_DT: NaiveDateTime = NaiveDateTime::MIN;

pub const MAX_DT: NaiveDateTime = NaiveDateTime::MAX;

#[allow(dead_code)]
pub const MIN_D: NaiveDate = NaiveDate::MIN;

#[allow(dead_code)]
pub const MAX_D: NaiveDate = NaiveDate::MAX;

/// 获取1970年1月1日的日期时间
/// - 无时区
pub fn start_dt() -> NaiveDateTime {
    let d = must_ok!(
        NaiveDate::parse_from_str("19700101", "%Y%m%d"),
        "Failed to create start date"
    );
    must_some!(d.and_hms_opt(0, 0, 0), "Failed to create start date time")
}

// # 与当前时间有关的函数
/// 获取当前日期时间字符串
/// - 无时区
/// - 格式：2000-01-01 00:00:00
#[allow(dead_code)]
pub fn now_dt_string() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[allow(dead_code)]
/// 获取当前日期字符串
/// - 无时区
/// - 格式：2000-01-01
pub fn now_d_string() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

/// 获取当前日期时间
/// - 无时区
pub fn now_dt() -> NaiveDateTime {
    Local::now().naive_local()
}

/// 仅获取当前日期
/// - 无时区
pub fn now_d() -> NaiveDate {
    Local::now().date_naive()
}

/// 获取当前年份
/// - 公元前为负值
pub fn now_year() -> i32 {
    Local::now().year()
}

// # 时间处理函数

#[allow(dead_code)]
/// 将无时区日期转化为字符串
/// - 格式：2000-01-01
pub fn to_d_string(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// 将无时区日期时间转化为字符串
/// - 格式：2000-01-01 00:00:00
pub fn to_dt_string(date_time: &NaiveDateTime) -> String {
    date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 将无时区日期时间转化为字符串
/// - 格式：
///     - 2000-01-01 00:00:00
///     - 01-01 00:00:00 如果今年和年份相等
pub fn to_omitted_dt_string(date_time: &NaiveDateTime) -> String {
    if Local::now().year() == date_time.year() {
        date_time.format("%m-%d %H:%M:%S").to_string()
    } else {
        date_time.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

/// 将字符串转化为无时区日期
/// - 格式: 20250101
pub fn parse_compact_ymd(ymd: &str) -> ArchiverResult<NaiveDate> {
    as_fatal!(NaiveDate::parse_from_str(ymd, "%Y%m%d"))
}

/// 找到这个月的最后一天
pub fn to_last_date(date: &NaiveDate) -> ArchiverResult<NaiveDate> {
    // 第一个unwrap可以直接使用
    let first_date = as_fatal!(date.with_day(1), "Failed to get first date")?;
    let next_month = as_fatal!(
        first_date.checked_add_months(Months::new(1)),
        "Fail to add month"
    )?;

    let last_date = as_fatal!(next_month.pred_opt(), "Fail to get pred date")?;
    Ok(last_date)
}
