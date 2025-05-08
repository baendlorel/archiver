use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<ArchiverCommand>,
}

#[derive(Subcommand)]
pub enum ArchiverCommand {
    /// Archive a file/directory by its name
    #[command(visible_alias = "a")]
    Archive {
        /// The name to archive
        #[arg(value_name = "name")]
        target: String,
    },

    /// Restore an archived object by its file/directory name or id
    #[command(visible_alias = "r")]
    Restore {
        /// The name or id of the object to restore
        #[arg(value_name = "name|id")]
        target: String,
    },

    /// Show the list of archived objects
    #[command(visible_alias = "ls")]
    List {
        /// The name or id of the object to archive
        #[arg(value_name = "name|id")]
        name: Option<String>,
    },

    /// Show the log of archiving operations
    #[command(visible_alias = "lg")]
    Log {
        /// Time interval like `YYYYMM YYYYMM` | `YYYYMM` or `YYYYMM *` | `* YYYYMM`
        #[arg(value_name = "time inverval")]
        time_interval: Option<String>,
    },
}
