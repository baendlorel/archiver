/// json序列化
/// - 用法：json.to各种string
mod json;
pub use json::SerdeJson;

/// bool序列化
/// - 配置在结构体字段的with上
/// - 用法：`#[serde(with = "boolean")]`
/// - 序列化为0或1
pub mod boolean;

/// NaiveDateTime序列化
/// - 配置在结构体字段的with上
/// - 用法：`#[serde(with = "naive_date_time")]`
/// - 序列化格式为"YYYY-MM-DD HH:MM:SS"
pub mod naive_date_time;

pub mod opt;
