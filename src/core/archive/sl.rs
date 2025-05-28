use crate::wrap_result;

use crate::misc::{jsonl, paths};
use crate::models::{error::ArchiverResult, types::ListEntry};

pub fn save(list: &[ListEntry]) -> ArchiverResult<()> {
    wrap_result!(jsonl::save(list, paths::LIST_FILE_PATH.as_path()))?;
    Ok(())
}
