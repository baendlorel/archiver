use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};

pub fn now_str() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn now_naive() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn now_naive_d() -> NaiveDate {
    Local::now().date_naive()
}

pub fn now_year() -> i32 {
    Local::now().year()
}
