use serde::{Deserialize, Serialize};

/// 定义用于序列化到JSON的日志条目结构
#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    pub time: String,   // 操作时间
    pub status: String, // 是否成功
    pub oper: String,   // 操作类型
    pub arg: String,    // 操作参数
    pub id: u32,        // archive id，如果有的话
}
