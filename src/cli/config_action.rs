use clap::Subcommand;

use crate::{cli::Operation, map};

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
                let opts = if *remove {
                    Some(map!("remove".to_string() => serde_json::Value::Bool(*remove)))
                } else {
                    None
                };
                Operation::new("cfg", "alias", vec![entry.clone()], opts)
            }
            ConfigAction::UpdateCheck { status } => Operation::new(
                "cfg",
                Some("update-check"),
                Some(vec![status.clone()]),
                None,
            ),
            ConfigAction::VaultItemSep { sep } => {
                Operation::new("cfg", "vault-item-sep", vec![sep.clone()], None)
            }
        }
    }
}
