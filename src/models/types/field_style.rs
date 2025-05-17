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

pub fn grey(str: &String) -> String {
    str.fg_rgb::<142, 142, 142>().to_string()
}

pub fn target_color(target: &String, is_dir: bool) -> String {
    if is_dir {
        target.blue().to_string()
    } else {
        target.to_string()
    }
}
