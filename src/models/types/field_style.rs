use owo_colors::OwoColorize;

pub trait Grey {
    fn grey(&self) -> String;
}

impl Grey for String {
    fn grey(&self) -> String {
        self.fg_rgb::<142, 142, 142>().to_string()
    }
}

impl Grey for str {
    fn grey(&self) -> String {
        self.fg_rgb::<142, 142, 142>().to_string()
    }
}
