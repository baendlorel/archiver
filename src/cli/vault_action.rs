use crate::misc;
use clap::{Parser, Subcommand};

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
