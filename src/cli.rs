use clap::{Parser, Subcommand};

use crate::misc;

#[derive(Parser)]
#[command(name="Archiver", author=env!("CARGO_PKG_AUTHORS"), version=concat!("v", env!("CARGO_PKG_VERSION"), " by ", env!("CARGO_PKG_AUTHORS")), about="ReadMe: https://github.com/baendlorel/archiver/blob/main/README.md", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<ArchiverCommand>,
}

#[derive(Subcommand)]
pub enum ArchiverCommand {
    /// Archive a file/directory by its name
    #[command(visible_aliases = ["p"])]
    Put {
        /// The file/directory names to be archived
        #[arg(value_name = "targets", required = true)]
        targets: Vec<String>, // 改成 Vec<String>

        /// The reason why you archive it
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Restore an archived object by its file/directory name or id
    #[command(visible_aliases = ["r", "res", "rst"])]
    Restore {
        /// id of the target to be restored. Can be obtained by command `arv list`
        #[arg(value_name = "ids", required = true)]
        ids: Vec<u32>,
    },

    /// Move archived objects to a new vault
    #[command(visible_aliases = ["m", "mov"])]
    Move {
        /// id of the target to be restored. Can be obtained by command `arv list`
        #[arg(value_name = "ids", required = true)]
        ids: Vec<u32>,

        /// To which vault
        #[arg[short, long, required = true]]
        to: String,
    },

    /// Vault management
    #[command(subcommand, visible_aliases = ["v", "vlt"])]
    Vault(VaultAction),

    /// Show the list of archived objects
    #[command(visible_aliases = ["l", "ls"])]
    List {
        /// Show all archived objects
        #[arg[short, long, group = "list_options"]]
        all: bool,

        /// Show restored objects
        #[arg[short, long, group = "list_options"]]
        restored: bool,
    },

    /// Show the log of archiving operations
    #[command(visible_alias = "lg")]
    Log {
        /// YYYYMM (display logs of this month), YYYYMM-YYYYMM
        #[arg(value_name = "time-range")]
        range: Option<String>,
    },

    // todo config也要像vault一样改为子命令比较好
    // todo config增加配置list列表项的功能
    /// Set or show configurations, use `arv config -h` to see more
    #[command(visible_aliases = ["c", "cfg"])]
    Config {
        #[arg(num_args = 0..=10, long_help=misc::CONFIG_HELP_TEXT,group = "config_options")]
        statement: Option<Vec<String>>,
    },

    #[command(visible_aliases = ["u", "up"])]
    Update,
}

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
