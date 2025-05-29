use crate::map;

use clap::Subcommand;
use serde_json::Value;

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
                Operation::new("vlt", Some("use"), None, Some(vec![name.clone()]), None)
            }
            VaultAction::Create { name, remark, u } => {
                let mut opts = map![];
                if let Some(remark) = remark {
                    opts.insert("remark".to_string(), Value::String(remark.clone()));
                }
                if *u {
                    opts.insert("use".to_string(), Value::String(String::new()));
                }
                let opts = if opts.len() == 0 { None } else { Some(opts) };
                Operation::new("vlt", Some("create"), None, Some(vec![name.clone()]), opts)
            }
            VaultAction::Remove { name } => {
                Operation::new("vlt", Some("remove"), None, Some(vec![name.clone()]), None)
            }
            VaultAction::List => Operation::new("vlt", Some("list"), None, None, None),
        }
    }
}
