use crate::{oper, opt_map};

use clap::Subcommand;

use crate::cli::Operation;
use crate::cli::short::main;

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show all configuration of Archiver
    #[command(aliases = ["l", "ls", "show"])]
    List,

    /// Set alias entries
    Alias {
        #[arg(value_name = "alias=path", required = true)]
        entry: String,

        #[arg(short, long, help = "Remove alias")]
        remove: bool,
    },

    /// Set auto check update
    UpdateCheck {
        #[arg(value_name = "on/off")]
        status: String,
    },

    /// Set auto check update
    VaultItemSep {
        #[arg(value_name = "separator")]
        sep: String,
    },
}

impl ConfigAction {
    pub fn to_operation(&self) -> Operation {
        match self {
            ConfigAction::List => oper!(main::CONFIG, "list", None, None),
            ConfigAction::Alias { entry, remove } => {
                oper!(main::CONFIG, "alias", [entry], opt_map![remove])
            }
            ConfigAction::UpdateCheck { status } => {
                oper!(main::CONFIG, "update-check", [status], None)
            }
            ConfigAction::VaultItemSep { sep } => {
                oper!(main::CONFIG, "vault-item-sep", [sep], None)
            }
        }
    }
}
