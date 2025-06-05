use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::misc::{clap_mark, mark};

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
    pub fn to_mark(&self) -> String {
        match self {
            LogLevel::Success => mark::succ(),
            LogLevel::Info => mark::info(),
            LogLevel::Warn => mark::warn(),
            LogLevel::Error => mark::error(),
            LogLevel::Fatal => mark::fatal(),
        }
    }

    pub fn to_clap_mark(&self) -> String {
        match self {
            LogLevel::Success => clap_mark::succ(),
            LogLevel::Info => clap_mark::info(),
            LogLevel::Warn => clap_mark::warn(),
            LogLevel::Error => clap_mark::error(),
            LogLevel::Fatal => clap_mark::fatal(),
        }
    }

    pub fn to_display(&self) -> String {
        match self {
            LogLevel::Success => "success".green().bold().to_string(),
            LogLevel::Info => "info".cyan().bold().to_string(),
            LogLevel::Warn => "warn".yellow().bold().to_string(),
            LogLevel::Error => "error".red().bold().to_string(),
            LogLevel::Fatal => "fatal".red().bold().underline().to_string(),
        }
    }
}
