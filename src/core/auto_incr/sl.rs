use crate::as_fatal;

use std::fs;

use crate::{
    misc::paths,
    models::{error::ArchiverError, serde_json::SerdeJson, types::AutoIncr},
};

pub fn save(auto_incr: &AutoIncr) -> Result<(), ArchiverError> {
    let json = as_fatal!(auto_incr.to_formatted_string())?;
    as_fatal!(fs::write(paths::AUTO_INCR_FILE_PATH.as_path(), json))?;
    Ok(())
}

pub fn load() -> Result<AutoIncr, ArchiverError> {
    let auto_incr_file = paths::AUTO_INCR_FILE_PATH.as_path();
    let content = as_fatal!(fs::read_to_string(auto_incr_file))?;
    let auto_incr = as_fatal!(AutoIncr::from_json_string(&content))?;
    Ok(auto_incr)
}
