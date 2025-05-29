pub mod dt;
pub mod paths;
pub mod rand;

mod confirm;
mod consts;
mod containers;

pub mod jsonl;
pub use confirm::confirm;
pub use consts::mark;
pub use containers::dedup_and_log;
