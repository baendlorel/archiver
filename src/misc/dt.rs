use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};

use crate::{models::error::ArchiverError, wrap_err_fatal};

// # 日期常量
pub const MIN_DT: NaiveDateTime = NaiveDateTime::MIN;
pub const MAX_DT: NaiveDateTime = NaiveDateTime::MAX;
pub const MIN_D: NaiveDate = NaiveDate::MIN;
pub const MAX_D: NaiveDate = NaiveDate::MAX;

// # 与当前时间有关的函数
/// 获取当前日期时间字符串
/// - 无时区
/// - 格式：2000-01-01 00:00:00
pub fn now_dt_string() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

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

/// 将字符串转化为无时区日期
/// - 格式: 200012
pub fn parse_from_ym(ym: &str) -> Result<NaiveDate, ArchiverError> {
    wrap_err_fatal!(NaiveDate::parse_from_str(ym, "%Y%m"))
}
