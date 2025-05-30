use owo_colors::OwoColorize;

pub trait CustomColors: std::fmt::Display {
    fn bright_grey(&self) -> String {
        self.fg_rgb::<170, 170, 170>().to_string()
    }

    /// rgb(255, 165, 0)
    fn orange(&self) -> String {
        self.fg_rgb::<255, 165, 0>().to_string()
    }

    /// rgb(146, 100, 14)
    fn dimmed_orange(&self) -> String {
        self.fg_rgb::<146, 100, 14>().to_string()
    }

    /// rgb(194, 241, 118)
    fn styled_vault(&self) -> String {
        self.fg_rgb::<194, 241, 118>().to_string()
    }

    /// rgb(170, 239, 58)
    fn styled_vault_item_seperator(&self) -> String {
        self.fg_rgb::<170, 239, 58>().to_string()
    }

    /// rgb(255, 80, 164)
    fn styled_archive_id(&self) -> String {
        self.fg_rgb::<255, 80, 164>().to_string()
    }

    /// rgb(43, 225, 201)
    fn styled_dir(&self) -> String {
        self.fg_rgb::<43, 225, 201>().bold().to_string()
    }

    /// rgb(159,221,254)
    fn styled_config_field(&self) -> String {
        self.fg_rgb::<159, 221, 254>().to_string()
    }

    /// rgb(81,195,255)
    fn styled_const(&self) -> String {
        self.fg_rgb::<81, 195, 255>().to_string()
    }

    /// rgb(207,148,124)
    fn styled_json_string(&self) -> String {
        self.fg_rgb::<207, 148, 124>().to_string()
    }
}

impl CustomColors for String {}
impl CustomColors for str {}
impl CustomColors for u32 {}
