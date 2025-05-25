use owo_colors::OwoColorize;

pub trait CustomColors: std::fmt::Display {
    fn grey(&self) -> String {
        self.fg_rgb::<142, 142, 142>().to_string()
    }
    fn orange(&self) -> String {
        self.fg_rgb::<255, 165, 0>().to_string()
    }
}

impl CustomColors for String {}

impl CustomColors for str {}
