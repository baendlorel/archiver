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

    /// rgb(194, 241, 118)
    fn colored_vault(&self) -> String {
        self.fg_rgb::<194, 241, 118>().to_string()
    }

    /// rgb(255, 80, 164)
    fn colored_archive_id(&self) -> String {
        self.fg_rgb::<255, 80, 164>().to_string()
    }

    /// rgb(43, 225, 201)
    fn colored_dir(&self) -> String {
        self.fg_rgb::<43, 225, 201>().to_string()
    }
}

impl CustomColors for String {}

impl CustomColors for str {}

impl CustomColors for u32 {}
