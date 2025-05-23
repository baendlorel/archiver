use std::path;

use owo_colors::OwoColorize;

// use crate::misc::paths::CURRENT_ID;
// pub fn id_to_str(id: u32) -> String {
//     let id_len = CURRENT_ID.to_string().len();
//     format!(
//         "{}{:0>id_len$}{}",
//         "", // id:
//         id.magenta(),
//         "".fg_rgb::<142, 172, 142>(),
//         id_len = id_len,
//     )
// }

pub fn grey(str: &str) -> String {
    str.fg_rgb::<142, 142, 142>().to_string()
}

pub fn target_color(target: &str, is_dir: bool) -> String {
    if is_dir {
        format!("{}{}", target.blue(), path::MAIN_SEPARATOR)
    } else {
        target.to_string()
    }
}
