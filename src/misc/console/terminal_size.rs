use terminal_size::{Width, terminal_size};

pub fn get_terminal_width() -> usize {
    if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        80 // 默认宽度
    }
}
