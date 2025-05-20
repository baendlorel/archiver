use clap::{Parser, Subcommand};

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
        #[arg[short,long]]
        all: bool,
    },

    /// Show the log of archiving operations
    #[command(visible_alias = "lg")]
    Log {
        /// YYYYMM (from yyyymm to now), YYYYMM-YYYYMM (date range), *-YYYYMM
        #[arg(value_name = "time-range")]
        range: Option<String>,
    },

    /// Configurations
    #[command(visible_aliases = ["c", "cfg"])]
    Config {
        /// Example: `--alias /usr/bin=@bin`. Can shorten the paths showed in console, but full paths are still preserved on records.
        #[arg(num_args = 0..=10, group = "config_options")]
        statement: Option<Vec<String>>,
    },

    #[command(visible_aliases = ["u", "up"])]
    Update,
}
