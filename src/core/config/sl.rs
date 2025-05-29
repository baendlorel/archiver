use crate::as_fatal;

use std::fs;

use crate::misc::paths;
use crate::models::{error::ArchiverResult, serde_custom::SerdeJson, types::ArchiverConfig};

pub fn save(config: &ArchiverConfig) -> ArchiverResult<()> {
    let json = as_fatal!(config.to_formatted_string())?;
    as_fatal!(fs::write(paths::CONFIG_FILE_PATH.as_path(), json))?;
    Ok(())
}
