pub mod console;
pub mod dt;
pub mod jsonl;
pub mod paths;
pub mod rand;

mod consts;
mod containers;

pub use consts::mark;
pub use containers::dedup_and_log;
