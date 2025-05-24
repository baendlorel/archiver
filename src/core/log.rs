use crate::{log_if_err, wrap_result};

use crate::misc::mark;
use crate::models::error::ArchiverError;
use crate::models::types::OperType;

mod parser;
mod sl;

pub fn succ(oper: &OperType, arg: &str, id: Option<u32>, msg: &String) {
    println!("{} {}", mark::succ(), msg);
    log_if_err!(sl::save(oper, arg, true, id, None));
}

pub fn err(oper: &OperType, arg: &str, id: Option<u32>, e: ArchiverError) {
    e.display();
    log_if_err!(sl::save(oper, arg, false, id, Some(e.to_string())));
}

pub fn display(range: &Option<String>) -> Result<(), ArchiverError> {
    let (logs, reach_casual_limit, casual_limit) = wrap_result!(sl::load(range))?;
    logs.iter().rev().for_each(|l| println!("{}", l));
    if reach_casual_limit {
        println!("Recent {} logs displayed.", casual_limit);
    }
    Ok(())
}
