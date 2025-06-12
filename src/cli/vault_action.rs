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
    #[command(visible_aliases = ["rm"])]
    Remove {
        #[arg(value_name = "name", required = true)]
        name: String,
    },

    /// Recover a removed vault
    Recover {
        #[arg(value_name = "name", required = true)]
        name: String,
    },

    /// Rename a vault
    Rename {
        #[arg(value_name = "old_name", required = true)]
        old_name: String,

        #[arg(value_name = "new_name", required = true)]
        new_name: String,
    },

    /// List all vaults
    #[command(visible_aliases = ["ls"])]
    List {
        /// Show only valid vaults
        #[arg(short, long)]
        all: bool,
    },
}

macro_rules! va_oper {
    ($($args:tt)*) => {
        oper!(main::VAULT, $($args)*)
    };
}

type VA = VaultAction;
impl VaultAction {
    pub fn to_operation(&self) -> Operation {
        match self {
            VA::Use { name } => va_oper!("use", [name], None),
            VA::Create {
                name,
                remark,
                activate,
            } => va_oper!("create", [name], opt_map![remark, activate]),
            VA::Remove { name } => va_oper!("remove", [name], None),
            VA::Recover { name } => va_oper!("recover", [name], None),
            VA::Rename { old_name, new_name } => va_oper!("rename", [old_name, new_name], None),
            VA::List { all } => va_oper!("list", None, opt_map![all]),
        }
    }
}
