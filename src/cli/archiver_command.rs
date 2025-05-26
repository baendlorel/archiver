use crate::map;

use clap::Subcommand;
use serde_json::Value;
use std::{collections::HashMap, vec};

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
    pub fn to_operation(&self) -> Operation {
        match self {
            ArchiverCommand::Put { targets, message } => {
                let opts = if let Some(m) = message {
                    map!["message".to_string() => Value::String(m.clone())]
                } else {
                    map![]
                };
                Operation::simple("put", targets.clone(), opts)
            }
            ArchiverCommand::Restore { ids } => Operation::simple(
                "rst",
                ids.iter().map(|id| id.to_string()).collect::<Vec<String>>(),
                map![],
            ),
            ArchiverCommand::Move { ids, to } => Operation::simple(
                "mov",
                ids.iter().map(|id| id.to_string()).collect::<Vec<String>>(),
                map!["to".to_string() => Value::String(to.clone())],
            ),
            ArchiverCommand::Vault(action) => action.to_operation(),
            ArchiverCommand::List { all, restored } => Operation::simple("lst", vec![], {
                let mut opts: HashMap<String, Value> = HashMap::new();
                if *all {
                    opts.insert("all".to_string(), Value::Bool(true));
                }
                if *restored {
                    opts.insert("restored".to_string(), Value::Bool(true));
                }
                opts
            }),
            ArchiverCommand::Log { range } => {
                let args = if let Some(range) = range {
                    vec![range.clone()]
                } else {
                    vec![]
                };
                Operation::simple("log", args, map![])
            }
            ArchiverCommand::Config { statement } => {
                Operation::new("cfg", "", vec![], HashMap::new())
            }
            ArchiverCommand::Update => Operation::simple("upd", vec![], HashMap::new()),
        }
    }
}
