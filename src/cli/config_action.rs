use clap::Subcommand;

use crate::{cli::Operation, opt_map};

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
            ConfigAction::List => Operation::new("cfg", "list", None, None),
            ConfigAction::Alias { entry, remove } => {
                Operation::new("cfg", "alias", vec![entry.clone()], opt_map![remove])
            }
            ConfigAction::UpdateCheck { status } => {
                Operation::new("cfg", "update-check", vec![status.clone()], None)
            }
            ConfigAction::VaultItemSep { sep } => {
                Operation::new("cfg", "vault-item-sep", vec![sep.clone()], None)
            }
        }
    }
}
