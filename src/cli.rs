use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<ArchiverCommand>,
}

#[derive(Subcommand)]
pub enum ArchiverCommand {
    /// Show the list of archived objects
    #[command(visible_alias = "ls")]
    List,

    /// Show the log of archiving operations
    #[command(visible_alias = "lg")]
    Log,

    /// Restore an archived object by its file/directory name or id
    #[command(visible_alias = "r")]
    Restore {
        /// The name or id of the object to restore
        #[arg(value_name = "name|id")]
        target: String,
    },

    /// Archive a directory or file by its path
    #[command(visible_alias = "a")]
    Archive {
        /// The path to archive
        #[arg(value_name = "target")]
        target: String,
    },
}
