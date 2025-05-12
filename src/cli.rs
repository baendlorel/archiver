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
    #[command(visible_alias = "r")]
    Restore {
        /// The name or id of the object to restore
        #[arg(value_name = "id")]
        id: u32,
    },

    /// Show the list of archived objects
    #[command(visible_alias = "ls")]
    List {
        #[arg[short,long]]
        all: bool,
    },

    /// Show the log of archiving operations
    #[command(visible_alias = "lg")]
    Log {
        /// Time interval like `YYYYMM YYYYMM` | `YYYYMM` or `YYYYMM *` | `* YYYYMM`
        #[arg(value_name = "time inverval")]
        interval: Option<String>,
    },
}
