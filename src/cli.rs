use clap::{Parser, Subcommand};

use crate::misc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<ArvCmd>,
}

#[derive(Subcommand)]
pub enum ArvCmd {
    /// Archive a file/directory by its name
    #[command(visible_aliases = ["p"])]
    Put {
        /// The file/directory names to be archived.
        #[arg(value_name = "targets", required = true)]
        targets: Vec<String>, // 改成 Vec<String>
    },

    /// Restore an archived object by its file/directory name or id
    #[command(visible_aliases = ["r", "res", "rst"])]
    Restore {
        /// id of the target to be restored. Can be obtained by command `arv list`
        #[arg(value_name = "ids", required = true)]
        ids: Vec<u32>,
    },

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
        /// YYYYMM (from yyyymm to now), YYYYMM-YYYYMM, *-YYYYMM
        #[arg(value_name = "time-range")]
        range: Option<String>,
    },

    /// Set or show configurations, use `arv config -h` to see more
    #[command(visible_aliases = ["c", "cfg"])]
    Config {
        #[arg(num_args = 0..=10, long_help=misc::CONFIG_HELP_TEXT,group = "config_options")]
        statement: Option<Vec<String>>,
    },

    #[command(visible_aliases = ["u", "up"])]
    Update,
}
