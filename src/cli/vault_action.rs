use crate::{oper, opt_map};

use clap::Subcommand;

use crate::cli::{Operation, short::main};

#[derive(Subcommand)]
pub enum VaultAction {
    /// Use a vault by name
    Use {
        #[arg(value_name = "name", required = true)]
        name: String,
    },

    /// Create a new vault
    Create {
        /// Name of your new vault
        #[arg(value_name = "name", required = true)]
        name: String,

        /// Optional remark for the vault
        #[arg(short, long)]
        remark: Option<String>,

        /// Use the new vault at once
        #[arg(short, long)]
        activate: bool,
    },

    /// Remove a vault by name
    Remove {
        #[arg(value_name = "name", required = true)]
        name: String,
    },

    /// List all vaults
    #[command(visible_aliases = ["ls"])]
    List,
}

impl VaultAction {
    pub fn to_operation(&self) -> Operation {
        match self {
            VaultAction::Use { name } => oper!(main::VAULT, "use", [name], None),
            VaultAction::Create {
                name,
                remark,
                activate,
            } => oper!(main::VAULT, "create", [name], opt_map![remark, activate]),
            VaultAction::Remove { name } => {
                oper!(main::VAULT, "remove", [name], None)
            }
            VaultAction::List => oper!(main::VAULT, "list", None, None),
        }
    }
}
