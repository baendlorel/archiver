use crate::map;

use clap::Subcommand;
use serde_json::Value;
use std::collections::HashMap;

use crate::models::types::Operation;

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
        #[arg(short)]
        u: bool,
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
    pub fn to_operation(&self) -> crate::models::types::Operation {
        match self {
            VaultAction::Use { name } => {
                Operation::new("vlt", "use", "", vec![name.clone()], map![])
            }
            VaultAction::Create { name, remark, u } => {
                let mut opts = HashMap::new();
                if let Some(remark) = remark {
                    opts.insert("remark".to_string(), Value::String(remark.clone()));
                }
                if *u {
                    opts.insert("use".to_string(), Value::String(String::new()));
                }
                Operation::new("vlt", "create", "", vec![name.clone()], opts)
            }
            VaultAction::Remove { name } => {
                Operation::new("vlt", "remove", "", vec![name.clone()], HashMap::new())
            }
            VaultAction::List => Operation::new("vlt", "list", "", vec![], HashMap::new()),
        }
    }
}
