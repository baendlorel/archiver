use crate::{oper, opt_map};

use clap::Subcommand;

use super::{config_action::ConfigAction, short::main, vault_action::VaultAction};
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
    #[command(visible_aliases = ["r", main::RESTORE])]
    Restore {
        /// id to be restored. Can be obtained by command `arv list`
        #[arg(value_name = "ids", required = true)]
        ids: Vec<u32>,
    },

    /// Move archived items to a new vault
    #[command(visible_aliases = ["m", "mv", main::MOVE])]
    Move {
        /// id to be moved. Can be obtained by command `arv list`
        #[arg(value_name = "ids", required = true)]
        ids: Vec<u32>,

        /// To which vault
        #[arg[short, long, required = true]]
        to: String,
    },

    /// Vault management
    #[command(subcommand, visible_aliases = ["v", main::VAULT])]
    Vault(VaultAction),

    /// Show the list of archived items
    #[command(visible_aliases = ["l", main::LIST])]
    List {
        /// Show all archived items
        #[arg[short, long, group = "list_options"]]
        all: bool,

        /// Show restored items
        #[arg[short, long, group = "list_options"]]
        restored: bool,

        /// Choose which vault to list
        #[arg[short, long]]
        vault: Option<String>,
    },

    /// Show the log of archiving operations
    #[command(visible_alias = main::LOG)]
    Log {
        /// YYYYMM (display logs of this month), YYYYMM-YYYYMM
        #[arg(value_name = "time-range")]
        range: Option<String>,

        #[arg(short, long)]
        id: Option<u32>,
    },

    /// Set or show configurations, use `arv config -h` to see more
    #[command(subcommand, visible_aliases = ["c", main::CONFIG])]
    Config(ConfigAction),

    /// Check for the updates. And update Archiver when there is a newer version
    #[command(visible_aliases = ["u", main::UPDATE])]
    Update,

    /// Check whether the core data is logically correct. By default, only failed entries are displayed
    #[command(visible_aliases = [main::CHECK])]
    Check {
        /// Show every checked items, including the passed
        #[arg(short, long)]
        verbose: bool,
    },
}

type C = ArchiverCommand;
impl ArchiverCommand {
    pub fn to_operation(&self) -> Operation {
        match self {
            C::Put {
                items,
                message,
                vault,
            } => oper!(main::PUT, items, opt_map![message, vault]),
            C::Restore { ids } => oper!(main::RESTORE, ids),
            C::Move { ids, to } => oper!(main::MOVE, ids, opt_map![to]),
            C::Vault(action) => action.to_operation(),
            C::List {
                all,
                restored,
                vault,
            } => oper!(main::LIST, None, opt_map![all, restored, vault]),
            C::Log { range, id } => oper!(main::LOG, [range], opt_map![id]),
            C::Config(action) => action.to_operation(),
            C::Update => oper!(main::UPDATE),
            C::Check { verbose } => oper!(main::CHECK, None, opt_map![verbose]),
        }
    }
}
