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
    if let Err(e) = save(oper, arg, true, id, None) {
        e.display();
    }
}

pub fn err(oper: &OperType, arg: &str, id: Option<u32>, e: ArchiverError) {
    e.display();
    // todo 报了warn错误却没有记录log，只有“~/.archiver/ ”
    if let Err(save_err) = save(oper, arg, false, id, Some(e.to_string())) {
        save_err.display();
    }
}
