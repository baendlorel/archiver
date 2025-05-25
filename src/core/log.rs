use crate::{log_if_err, wrap_result};

use crate::misc::mark;
use crate::models::error::ArchiverError;
use crate::models::types::OperType;

mod parser;
mod sl;

pub mod format_arg;

pub fn succ(
    oper: &OperType,
    arg: &str,
    archive_id: Option<u32>,
    vault_id: Option<u32>,
    msg: &String,
) {
    println!("{} {}", mark::succ(), msg);
    log_if_err!(sl::save(oper, arg, true, archive_id, vault_id, None));
}

pub fn err(oper: &OperType, arg: &str, e: ArchiverError) {
    e.display();
    log_if_err!(sl::save(oper, arg, false, None, None, Some(e.to_string())));
}

pub fn display(range: &Option<String>) -> Result<(), ArchiverError> {
    let (logs, reach_casual_limit, casual_limit) = wrap_result!(sl::load(range))?;
    logs.iter().rev().for_each(|l| println!("{}", l));
    if reach_casual_limit {
        println!("Recent {} logs displayed.", casual_limit);
    }
    Ok(())
}
