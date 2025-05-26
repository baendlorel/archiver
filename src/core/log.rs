use crate::{log_if_err, wrap_result};

use crate::misc::mark;
use crate::models::error::ArchiverError;

mod parser;
mod sl;

pub fn succ(archive_id: Option<u32>, vault_id: Option<u32>, msg: &String) {
    println!("{} {}", mark::succ(), msg);
    log_if_err!(sl::save(true, archive_id, vault_id, None));
}

pub fn fail(e: ArchiverError) {
    e.display();
    log_if_err!(sl::save(false, None, None, Some(e.to_string())));
}

pub fn display(range: &Option<String>) -> Result<(), ArchiverError> {
    let (logs, reach_casual_limit, casual_limit) = wrap_result!(sl::load(range))?;
    logs.iter()
        .rev()
        .for_each(|l| println!("{}", l.to_display()));
    if reach_casual_limit {
        println!("Recent {} logs displayed.", casual_limit);
    }
    Ok(())
}
