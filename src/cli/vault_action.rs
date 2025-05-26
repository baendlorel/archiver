use clap::Subcommand;

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
    /// Returns the name of the vault action
    pub fn get(&self) -> String {
        match self {
            VaultAction::Use { name } => format!("use {}", name),
            VaultAction::Create { name, .. } => format!("create {}", name),
            VaultAction::Remove { name } => format!("remove {}", name),
            VaultAction::List => "list".to_string(),
        }
    }

    pub fn to_operation(&self) -> crate::models::types::Operation {
        use crate::models::types::Operation;
        use std::collections::HashMap;
        match self {
            VaultAction::Use { name } => Operation::new("vlt", "use", name, HashMap::new()),
            VaultAction::Create { name, remark, u } => {
                let mut opts = HashMap::new();
                if let Some(r) = remark {
                    opts.insert("remark".to_string(), r.clone());
                }
                if *u {
                    opts.insert("u".to_string(), "true".to_string());
                }
                Operation::new("vlt", "create", name, opts)
            }
            VaultAction::Remove { name } => Operation::new("vlt", "remove", name, HashMap::new()),
            VaultAction::List => Operation::new("vlt", "list", "", HashMap::new()),
        }
    }
}
