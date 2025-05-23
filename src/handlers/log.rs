use crate::log_if_err;
use crate::misc::mark;
use crate::models::error::ArchiverError;
use crate::models::types::OperType;

mod parse_range;
use parse_range::parse_range;
mod sl;

pub fn handler(range: &Option<String>) {
    log_if_err!(sl::load(range));
}

pub fn succ(oper: &OperType, arg: &str, id: Option<u32>, msg: &String) {
    println!("{} {}", mark::succ(), msg);
    log_if_err!(sl::save(oper, arg, true, id, None));
}

pub fn err(oper: &OperType, arg: &str, id: Option<u32>, e: ArchiverError) {
    e.display();
    log_if_err!(sl::save(oper, arg, false, id, Some(e.to_string())));
}
