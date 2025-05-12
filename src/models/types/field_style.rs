use owo_colors::OwoColorize;

pub fn id_to_str(id: u32) -> String {
    format!(
        "{}{:03}{}",
        "id:",
        id.magenta(),
        "".fg_rgb::<142, 172, 142>()
    )
}

pub fn grey(str: &String) -> String {
    str.fg_rgb::<142, 142, 142>().to_string()
}

pub fn cwd(str: &String) -> String {
    str.italic().fg_rgb::<142, 142, 142>().to_string()
}

pub fn dir_color(target: &String, is_dir: bool) -> String {
    if is_dir {
        let mut str = target.blue().to_string();
        str.push('/');
        str
    } else {
        target.to_string()
    }
}
