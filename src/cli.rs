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
    #[command(visible_alias = "a")]
    Archive {
        /// The name to archive
        #[arg(value_name = "name")]
        target: String,
    },

    /// Restore an archived object by its file/directory name or id
    #[command(visible_aliases = ["r", "res", "rst"])]
    Restore {
        /// id of the target to be restored. Can be obtained by command `arv list`
        #[arg(value_name = "id")]
        id: u32,
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
        /// YYYYMM=to now, YYYYMM-YYYYMM=date range, *-YYYYMM= to ym.
        #[arg(value_name = "time range")]
        range: Option<String>,
    },

    /// Configurations
    #[command()]
    Config {
        /// Example: `--alias /usr/bin=@bin`. Can shorten the paths showed in console, but full paths are still preserved on records.
        #[arg(long, group = "config_options")]
        alias: Option<String>,

        #[arg(long, group = "config_options")]
        show_alias: bool,
    },

    /// It is a dangerous operation and we will not implement it. If you really want to clear the archive, just remove the '.archive' folder in your home dir.
    #[command()]
    Clear,
}
