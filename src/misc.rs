pub mod console;
pub mod dt;
pub mod jsonl;
pub mod paths;
pub mod rand;

mod consts;
mod containers;

pub use consts::{clap_mark, mark};
pub use containers::{dedup_and_log, some_to_map, some_to_vec};
