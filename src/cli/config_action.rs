use clap::Subcommand;

use crate::cli::Operation;

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show all configuration of Archiver
    #[command(aliases = ["l", "ls"])]
    List,

    /// Set alias entries
    #[command(subcommand)]
    Alias(AliasAction),

    /// Set auto check update
    #[command(subcommand)]
    AutoCheckUpdate(AutoCheckUpdateAction),

    /// Set auto check update
    #[command(subcommand)]
    VaultItemSeperator(VaultItemSeperatorAction),
}

#[derive(Subcommand)]
pub enum AliasAction {
    /// Set an alias-origin map
    Set {
        #[arg(value_name = "alias", required = true)]
        alias: String,
    },

    /// Remove an alias-origin map
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

#[derive(Subcommand)]
pub enum VaultItemSeperatorAction {
    /// Set auto check update on or off
    Set {
        #[arg(value_name = "sep", required = true)]
        seperator: String,
    },
}

impl ConfigAction {
    pub fn to_operation(&self) -> Operation {
        match self {
            ConfigAction::List => Operation::new("cfg", Some("list"), None, None, None),
            ConfigAction::Alias(action) => action.to_operation(),
            ConfigAction::AutoCheckUpdate(action) => action.to_operation(),
            ConfigAction::VaultItemSeperator(action) => action.to_operation(),
        }
    }
}

impl AliasAction {
    pub fn to_operation(&self) -> Operation {
        match self {
            AliasAction::Set { alias } => Operation::new(
                "cfg",
                Some("alias"),
                Some("set"),
                Some(vec![alias.clone()]),
                None,
            ),
            AliasAction::Remove { alias } => Operation::new(
                "cfg",
                Some("alias"),
                Some("remove"),
                Some(vec![alias.clone()]),
                None,
            ),
        }
    }
}

impl AutoCheckUpdateAction {
    pub fn to_operation(&self) -> Operation {
        match self {
            AutoCheckUpdateAction::Set { status } => Operation::new(
                "cfg",
                Some("auto-check-update"),
                Some("set"),
                Some(vec![status.clone()]),
                None,
            ),
        }
    }
}

impl VaultItemSeperatorAction {
    pub fn to_operation(&self) -> Operation {
        match self {
            VaultItemSeperatorAction::Set { seperator } => Operation::new(
                "cfg",
                Some("vault-item-seperator"),
                Some("set"),
                Some(vec![seperator.clone()]),
                None,
            ),
        }
    }
}
