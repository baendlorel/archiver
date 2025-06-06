use crate::{oper, opt_map};

use clap::Subcommand;

use crate::cli::Operation;
use crate::cli::short::main;

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show all configuration of Archiver
    #[command(aliases = ["l", "ls", "show"])]
    List {
        #[arg(short, long, help = "Show configs with comment")]
        comment: bool,
    },

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

macro_rules! ca_oper {
    ($($args:tt)*) => {
        oper!(main::CONFIG, $($args)*)
    };
}

type CA = ConfigAction;
impl ConfigAction {
    pub fn to_operation(&self) -> Operation {
        match self {
            CA::List { comment } => ca_oper!("list", None, opt_map![comment]),
            CA::Alias { entry, remove } => ca_oper!("alias", [entry], opt_map![remove]),
            CA::UpdateCheck { status } => ca_oper!("update-check", [status], None),
            CA::VaultItemSep { sep } => ca_oper!("vault-item-sep", [sep], None),
        }
    }
}
