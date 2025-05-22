use crate::misc::mark;
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
        e.display();
    }
}

pub fn succ(oper: &OperType, arg: &str, id: Option<u32>, msg: &String) {
    println!("{} {}", mark::succ(), msg);

    save(oper, arg, true, id, None)
        .map_err(|e| e.display())
        .ok();
}

pub fn err(oper: &OperType, arg: &str, id: Option<u32>, e: ArchiverError) {
    e.display();

    save(oper, arg, false, id, Some(e.to_string()))
        .map_err(|save_err| save_err.display())
        .ok();
}
