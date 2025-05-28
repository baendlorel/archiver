use serde::{Deserialize, Serialize};

use crate::misc::mark;

#[derive(Serialize, Deserialize, Clone)]
pub enum LogLevel {
    #[serde(rename = "s")]
    Success,

    #[serde(rename = "i")]
    Info,

    #[serde(rename = "w")]
    Warn,

    #[serde(rename = "e")]
    Error,

    #[serde(rename = "f")]
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

    pub fn to_string(&self) -> String {
        match self {
            LogLevel::Success => "SUCC",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
        .to_string()
    }
}
