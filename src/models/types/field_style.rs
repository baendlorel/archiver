use owo_colors::OwoColorize;

pub trait CustomColors: std::fmt::Display {
    fn bright_grey(&self) -> String {
        self.fg_rgb::<170, 170, 170>().to_string()
    }

    fn orange(&self) -> String {
        self.fg_rgb::<255, 165, 0>().to_string()
    }

    fn dimmed_orange(&self) -> String {
        self.fg_rgb::<146, 100, 14>().to_string()
    }

    /// rgb(146, 100, 14)
    fn colored_vault(&self) -> String {
        self.fg_rgb::<146, 100, 14>().to_string()
    }
}

impl CustomColors for String {}

impl CustomColors for str {}
