use serde::{Deserialize, Serialize};

use crate::misc::mark;

#[derive(Serialize, Deserialize, Clone)]
pub enum LogLevel {
    #[serde(rename = "S")]
    Success,

    #[serde(rename = "I")]
    Info,

    #[serde(rename = "W")]
    Warn,

    #[serde(rename = "E")]
    Error,

    #[serde(rename = "F")]
    Fatal,
}

impl LogLevel {
    pub fn is_succ(&self) -> bool {
        matches!(self, LogLevel::Success)
    }

    pub fn to_mark(&self) -> String {
        match self {
            LogLevel::Success => mark::succ(),
            LogLevel::Info => mark::info(),
            LogLevel::Warn => mark::warn(),
            LogLevel::Error => mark::error(),
            LogLevel::Fatal => mark::fatal(),
        }
    }
}
