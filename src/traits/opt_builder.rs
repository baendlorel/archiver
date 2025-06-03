use crate::cli::Opt;

pub trait OptBuilder {
    fn to_opt(&self) -> Opt
    where
        Self: std::fmt::Display,
    {
        Opt::String(self.to_string())
    }
}

impl OptBuilder for str {}
impl OptBuilder for &str {}
impl OptBuilder for String {}
impl OptBuilder for &String {}

impl OptBuilder for bool {
    fn to_opt(&self) -> Opt {
        Opt::Bool(*self)
    }
}

impl OptBuilder for u32 {
    fn to_opt(&self) -> Opt {
        Opt::U32(*self)
    }
}
