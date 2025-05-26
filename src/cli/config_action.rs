use crate::map;

use clap::Subcommand;

use crate::models::types::Operation;

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show all configuration of Archiver
    List,

    /// Set alias entries
    #[command(subcommand)]
    Alias(AliasAction),

    /// Set auto check update
    #[command(subcommand)]
    AutoCheckUpdate(AutoCheckUpdateAction),
}

#[derive(Subcommand)]
pub enum AliasAction {
    /// Add an alias entry
    Add {
        #[arg(value_name = "alias", required = true)]
        alias: String,
    },

    /// Remove an alias entry
    Remove {
        #[arg(value_name = "alias", required = true)]
        alias: String,
    },
}

#[derive(Subcommand)]
pub enum AutoCheckUpdateAction {
    /// Set auto check update on or off
    Set {
        #[arg(value_name = "on/off", required = true)]
        status: String,
    },
}

impl ConfigAction {
    pub fn to_operation(&self) -> Operation {
        match self {
            ConfigAction::List => Operation::new("cfg", "list", "", vec![], map![]),
            ConfigAction::Alias(action) => action.to_operation(),
            ConfigAction::AutoCheckUpdate(action) => action.to_operation(),
        }
    }
}

impl AliasAction {
    pub fn to_operation(&self) -> Operation {
        match self {
            AliasAction::Add { alias } => {
                Operation::new("cfg", "alias", "add", vec![alias.clone()], map![])
            }
            AliasAction::Remove { alias } => {
                Operation::new("cfg", "alias", "remove", vec![alias.clone()], map![])
            }
        }
    }
}

impl AutoCheckUpdateAction {
    pub fn to_operation(&self) -> Operation {
        match self {
            AutoCheckUpdateAction::Set { status } => Operation::new(
                "cfg",
                "auto-check-update",
                "set",
                vec![status.clone()],
                map![],
            ),
        }
    }
}
