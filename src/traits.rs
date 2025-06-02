mod field_style;
mod get_type;
mod no_loss_path;
mod result_ext;
mod strip_ansi;
mod to_option;

pub use field_style::CustomColors;
pub use get_type::{GetType, VarType};
pub use no_loss_path::ForceToString;
pub use result_ext::ResultExt;
pub use strip_ansi::StripAnsi;
pub use to_option::EnsureOptionExt;
