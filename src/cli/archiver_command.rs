use clap::Subcommand;
use std::collections::HashMap;

use super::vault_action::VaultAction;
use crate::{misc, models::types::Operation};

#[derive(Subcommand)]
pub enum ArchiverCommand {
    /// Archive a file/directory by its name
    #[command(visible_aliases = ["p"])]
    Put {
        /// The file/directory names to be archived
        #[arg(value_name = "targets", required = true)]
        targets: Vec<String>, // 改成 Vec<String>

        /// The reason why you archive it
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Restore an archived object by its file/directory name or id
    #[command(visible_aliases = ["r", "res", "rst"])]
    Restore {
        /// id of the target to be restored. Can be obtained by command `arv list`
        #[arg(value_name = "ids", required = true)]
        ids: Vec<u32>,
    },

    /// Move archived objects to a new vault
    #[command(visible_aliases = ["m", "mv", "mov"])]
    Move {
        /// id of the target to be restored. Can be obtained by command `arv list`
        #[arg(value_name = "ids", required = true)]
        ids: Vec<u32>,

        /// To which vault
        #[arg[short, long, required = true]]
        to: String,
    },

    /// Vault management
    #[command(subcommand, visible_aliases = ["v", "vlt"])]
    Vault(VaultAction),

    /// Show the list of archived objects
    #[command(visible_aliases = ["l", "ls"])]
    List {
        /// Show all archived objects
        #[arg[short, long, group = "list_options"]]
        all: bool,

        /// Show restored objects
        #[arg[short, long, group = "list_options"]]
        restored: bool,
    },

    /// Show the log of archiving operations
    #[command(visible_alias = "lg")]
    Log {
        /// YYYYMM (display logs of this month), YYYYMM-YYYYMM
        #[arg(value_name = "time-range")]
        range: Option<String>,
    },

    // todo config也要像vault一样改为子命令比较好
    /// Set or show configurations, use `arv config -h` to see more
    #[command(visible_aliases = ["c", "cfg"])]
    Config {
        #[arg(num_args = 0..=10, long_help=misc::CONFIG_HELP_TEXT,group = "config_options")]
        statement: Option<Vec<String>>,
    },

    #[command(visible_aliases = ["u", "up"])]
    Update,
}

// todo impl一个类似to_oper和arg的函数
impl ArchiverCommand {
    /// 获取当前操作的名称
    pub fn get_main_command(&self) -> &'static str {
        match self {
            ArchiverCommand::Put { .. } => "put",
            ArchiverCommand::Restore { .. } => "rst",
            ArchiverCommand::Move { .. } => "mov",
            ArchiverCommand::Vault(_) => "vlt",
            ArchiverCommand::List { .. } => "lst",
            ArchiverCommand::Log { .. } => "log",
            ArchiverCommand::Config { .. } => "cfg",
            ArchiverCommand::Update => "upd",
        }
    }

    pub fn get_arg(&self) -> String {
        match self {
            ArchiverCommand::Put { targets, .. } => targets.join(" "),
            ArchiverCommand::Restore { ids } => ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            ArchiverCommand::Move { ids, to } => format!(
                "ids: [{}], to: {}",
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                to
            ),
            ArchiverCommand::Vault(action) => action.to_operation(),
            ArchiverCommand::List { all, restored } => {
                format!("all: {}, restored: {}", all, restored)
            }
            ArchiverCommand::Log { range } => range.clone().unwrap_or_default(),
            ArchiverCommand::Config { statement } => {
                statement.as_ref().map_or(String::new(), |s| s.join(" "))
            }
            ArchiverCommand::Update => String::new(),
        }
    }

    pub fn to_operation(&self) -> String {
        match self {
            ArchiverCommand::Put { targets, message } => {
                Operation::new("put", "put", &targets.join(" "), {
                    let mut opts = HashMap::new();
                    if let Some(msg) = message {
                        opts.insert("message".to_string(), msg.clone());
                    }
                    opts
                })
            }
            ArchiverCommand::Restore { ids } => Operation::new(
                "rst",
                "restore",
                &ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                HashMap::new(),
            ),
            ArchiverCommand::Move { ids, to } => Operation::new(
                "mov",
                "move",
                &ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                {
                    let mut opts = HashMap::new();
                    opts.insert("to".to_string(), to.clone());
                    opts
                },
            ),
            ArchiverCommand::Vault(action) => action.to_operation(),
            ArchiverCommand::List { all, restored } => Operation::new("lst", "list", "", {
                let mut opts = HashMap::new();
                if *all {
                    opts.insert("all".to_string(), "true".to_string());
                }
                if *restored {
                    opts.insert("restored".to_string(), "true".to_string());
                }
                opts
            }),
            ArchiverCommand::Log { range } => Operation::new(
                "log",
                "log",
                &range.clone().unwrap_or_default(),
                HashMap::new(),
            ),
            ArchiverCommand::Config { statement } => Operation::new(
                "cfg",
                "config",
                &statement.as_ref().map_or(String::new(), |s| s.join(" ")),
                HashMap::new(),
            ),
            ArchiverCommand::Update => Operation::new("upd", "update", "", HashMap::new()),
        }
    }
}
