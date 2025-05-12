use owo_colors::OwoColorize;

pub fn id_to_str(id: u32) -> String {
    format!("{}{:03}{}", "id:", id.green(), "".fg_rgb::<142, 172, 142>())
}

pub fn grey(str: &String) -> String {
    str.fg_rgb::<142, 142, 142>().to_string()
}

pub fn cwd(str: &String) -> String {
    str.italic().fg_rgb::<142, 142, 142>().to_string()
}
