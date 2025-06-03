use crate::opt_map;

use clap::Subcommand;
use std::vec;

use super::{config_action::ConfigAction, short, vault_action::VaultAction};
use crate::cli::Operation;

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

        #[arg(short, long)]
        id: Option<u32>,
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
            } => Operation::simple(short::main::PUT, items.clone(), opt_map![message, vault]),
            ArchiverCommand::Restore { ids } => Operation::simple(
                short::main::RESTORE,
                ids.iter().map(|id| id.to_string()).collect::<Vec<String>>(),
                None,
            ),
            ArchiverCommand::Move { ids, to } => Operation::simple(
                short::main::MOVE,
                ids.iter().map(|id| id.to_string()).collect::<Vec<String>>(),
                opt_map![to],
            ),
            ArchiverCommand::Vault(action) => action.to_operation(),
            ArchiverCommand::List { all, restored } => {
                Operation::simple("lst", None, opt_map![all, restored])
            }
            ArchiverCommand::Log { range, id } => Operation::simple(
                short::main::LOG,
                range.clone().map(|r| vec![r]),
                opt_map![id],
            ),
            ArchiverCommand::Config(action) => action.to_operation(),
            ArchiverCommand::Update => Operation::simple(short::main::UPDATE, None, None),
        }
    }
}
