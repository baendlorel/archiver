use crate::{as_fatal, wrap_result};

use crate::misc::{jsonl, paths};
use crate::models::{error::ArchiverError, types::ListEntry};

pub fn save(list: &[ListEntry]) -> Result<(), ArchiverError> {
    wrap_result!(jsonl::save(list, paths::LIST_FILE_PATH.as_path()))?;
    Ok(())
}

pub fn load() -> Result<Vec<ListEntry>, ArchiverError> {
    let list = wrap_result!(jsonl::load::<ListEntry>(paths::LIST_FILE_PATH.as_path()))?;
    Ok(list)
}
