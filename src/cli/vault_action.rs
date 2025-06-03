use crate::{map, opt_map};

use clap::Subcommand;
use serde_json::Value;

use crate::cli::Operation;
use crate::traits::EnsureOption;

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
            VaultAction::Use { name } => Operation::new("vlt", "use", vec![name.clone()], None),
            VaultAction::Create {
                name,
                remark,
                activate: u,
            } => {
                let mut opts = map![];
                if let Some(remark) = remark {
                    opts.insert("remark".to_string(), Value::String(remark.clone()));
                }
                if *u {
                    opts.insert("use".to_string(), Value::String(String::new()));
                }
                let opts = opt_map![remark, u];
                Operation::new("vlt", "create", vec![name.clone()], opts)
            }
            VaultAction::Remove { name } => {
                Operation::new("vlt", "remove", vec![name.clone()], None)
            }
            VaultAction::List => Operation::new("vlt", "list", None, None),
        }
    }
}
