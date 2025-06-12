use owo_colors::OwoColorize;

pub trait CustomColors: std::fmt::Display {
    fn grey(&self) -> String {
        self.fg_rgb::<148, 148, 152>().to_string()
    }

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

    /// rgb(255, 80, 164)
    fn styled_id(&self) -> String {
        self.fg_rgb::<255, 80, 164>().to_string()
    }

    /// rgb(118, 255, 232)
    fn styled_sys_id(&self) -> String {
        self.fg_rgb::<118, 255, 232>().to_string()
    }

    /// rgb(255, 225, 115)
    fn styled_trans_id(&self) -> String {
        self.fg_rgb::<255, 225, 115>().to_string()
    }

    /// rgb(194, 241, 118)
    fn styled_vault(&self) -> String {
        self.fg_rgb::<194, 241, 118>().to_string()
    }

    /// rgb(170, 239, 58)
    fn styled_vault_item_sep(&self) -> String {
        self.fg_rgb::<170, 239, 58>().to_string()
    }

    /// rgb(43, 225, 201)
    fn styled_dir(&self) -> String {
        self.fg_rgb::<43, 225, 201>().bold().to_string()
    }

    /// rgb(159,221,254)
    fn styled_field(&self) -> String {
        self.fg_rgb::<159, 221, 254>().to_string()
    }

    /// rgb(81,195,255)
    fn styled_const(&self) -> String {
        self.fg_rgb::<81, 195, 255>().to_string()
    }

    /// rgb(207,148,124)
    fn styled_string(&self) -> String {
        self.fg_rgb::<207, 148, 124>().to_string()
    }

    fn styled_message(&self) -> String;

    /// rgb(110,156,90)
    fn styled_comment(&self) -> String {
        self.fg_rgb::<110, 156, 90>().to_string()
    }

    /// rgb(68, 190, 255)
    fn styled_valid(&self) -> String {
        self.fg_rgb::<68, 190, 255>().to_string()
    }

    /// rgb(255, 59, 59)
    fn styled_invalid(&self) -> String {
        self.fg_rgb::<255, 59, 59>().to_string()
    }
}

impl CustomColors for String {
    fn styled_message(&self) -> String {
        if self.is_empty() {
            "<none>".grey()
        } else {
            self.fg_rgb::<255, 255, 255>().to_string()
        }
    }
}
impl CustomColors for str {
    fn styled_message(&self) -> String {
        if self.is_empty() {
            "<none>".grey()
        } else {
            self.fg_rgb::<255, 255, 255>().to_string()
        }
    }
}

impl CustomColors for u32 {
    fn styled_message(&self) -> String {
        self.fg_rgb::<255, 255, 255>().to_string()
    }
}
