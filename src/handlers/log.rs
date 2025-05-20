use std::u32;

use crate::misc::status_mark;
use crate::models::error::ArchiverError;
use crate::models::types::OperType;

mod parse_range;
use parse_range::parse_range;
mod save;
use save::save;
mod load;
use load::load;

pub fn handler(range: &Option<String>) {
    if let Err(e) = load(range) {
        println!("{}", e.to_string())
    }
}

pub fn succ(oper: &OperType, arg: &str, id: Option<u32>, remark: Option<String>) {
    if let Err(e) = save(oper, arg, true, id, remark) {
        println!("{}", e.to_string())
    }
}

pub fn err(oper: &OperType, arg: &str, id: Option<u32>, e: ArchiverError) {
    let err_msg = e.to_string();
    println!("{} {}", status_mark::fail(), err_msg);
    if let Err(e) = save(oper, arg, false, id, Some(err_msg)) {
        println!("{}", e.to_string())
    }
}
