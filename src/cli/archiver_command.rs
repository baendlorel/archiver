use crate::map;

use clap::Subcommand;
use serde_json::Value;
use std::{collections::HashMap, vec};

use super::{config_action::ConfigAction, short, vault_action::VaultAction};
use crate::models::types::Operation;

#[derive(Subcommand)]
pub enum ArchiverCommand {
    /// Archive a file/directory by its name
    #[command(visible_aliases = ["p"])]
    Put {
        /// The file/directory names to be archived
        #[arg(value_name = "items", required = true)]
        items: Vec<String>, // 改成 Vec<String>

        /// The reason why you archive it
        #[arg(short, long)]
        message: Option<String>,

        /// To which vault, default to the current vault
        #[arg(short, long)]
        vault: Option<String>,
    },

    /// Restore an archived object by its file/directory name or id
    #[command(visible_aliases = ["r", short::main::RESTORE])]
    Restore {
        /// id to be restored. Can be obtained by command `arv list`
        #[arg(value_name = "ids", required = true)]
        ids: Vec<u32>,
    },

    /// Move archived objects to a new vault
    #[command(visible_aliases = ["m", "mv", short::main::MOVE])]
    Move {
        /// id to be moved. Can be obtained by command `arv list`
        #[arg(value_name = "ids", required = true)]
        ids: Vec<u32>,

        /// To which vault
        #[arg[short, long, required = true]]
        to: String,
    },

    /// Vault management
    #[command(subcommand, visible_aliases = ["v", short::main::VAULT])]
    Vault(VaultAction),

    /// Show the list of archived objects
    #[command(visible_aliases = ["l", short::main::LIST])]
    List {
        /// Show all archived objects
        #[arg[short, long, group = "list_options"]]
        all: bool,

        /// Show restored objects
        #[arg[short, long, group = "list_options"]]
        restored: bool,
    },

    /// Show the log of archiving operations
    #[command(visible_alias = short::main::LOG)]
    Log {
        /// YYYYMM (display logs of this month), YYYYMM-YYYYMM
        #[arg(value_name = "time-range")]
        range: Option<String>,
    },

    /// Set or show configurations, use `arv config -h` to see more
    #[command(subcommand, visible_aliases = ["c", short::main::CONFIG])]
    Config(ConfigAction),

    #[command(visible_aliases = ["u", short::main::UPDATE])]
    Update,
}

impl ArchiverCommand {
    pub fn to_operation(&self) -> Operation {
        match self {
            ArchiverCommand::Put {
                items,
                message,
                vault,
            } => {
                let mut opts: HashMap<String, Value> = map![];
                if let Some(m) = message {
                    opts.insert("message".to_string(), Value::String(m.clone()));
                }
                if let Some(v) = vault {
                    opts.insert("vault".to_string(), Value::String(v.clone()));
                }
                let opts = if opts.len() == 0 { None } else { Some(opts) };

                Operation::simple(short::main::PUT, Some(items.clone()), opts)
            }
            ArchiverCommand::Restore { ids } => Operation::simple(
                short::main::RESTORE,
                Some(ids.iter().map(|id| id.to_string()).collect::<Vec<String>>()),
                None,
            ),
            ArchiverCommand::Move { ids, to } => Operation::simple(
                short::main::MOVE,
                Some(ids.iter().map(|id| id.to_string()).collect::<Vec<String>>()),
                Some(map!["to".to_string() => Value::String(to.clone())]),
            ),
            ArchiverCommand::Vault(action) => action.to_operation(),
            ArchiverCommand::List { all, restored } => Operation::simple("lst", None, {
                let mut opts: HashMap<String, Value> = HashMap::new();
                if *all {
                    opts.insert("all".to_string(), Value::Bool(true));
                }
                if *restored {
                    opts.insert("restored".to_string(), Value::Bool(true));
                }
                if opts.len() == 0 { None } else { Some(opts) }
            }),
            ArchiverCommand::Log { range } => {
                let args = if let Some(range) = range {
                    Some(vec![range.clone()])
                } else {
                    None
                };
                Operation::simple(short::main::LOG, args, None)
            }
            ArchiverCommand::Config(action) => action.to_operation(),
            ArchiverCommand::Update => Operation::simple(short::main::UPDATE, None, None),
        }
    }
}
