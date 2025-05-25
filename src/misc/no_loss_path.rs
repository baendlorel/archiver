use std::path::{Path, PathBuf};

use crate::misc::mark;

pub trait ForceToString {
    fn force_to_string(&self) -> String;
}

impl ForceToString for Path {
    fn force_to_string(&self) -> String {
        force_no_loss(&self)
    }
}

impl ForceToString for PathBuf {
    fn force_to_string(&self) -> String {
        force_no_loss(&self)
    }
}

impl ForceToString for std::ffi::OsStr {
    fn force_to_string(&self) -> String {
        force_no_loss(&self)
    }
}

impl ForceToString for std::ffi::OsString {
    fn force_to_string(&self) -> String {
        force_no_loss(&self)
    }
}

fn force_no_loss<T: AsRef<Path>>(t: &T) -> String {
    // 先尝试严格转换
    if let Ok(s) = t.as_ref().as_os_str().to_os_string().into_string() {
        return s;
    }

    // 检查 to_string_lossy 是否引入 �
    let lossy = t.as_ref().to_string_lossy();
    if lossy.contains('\u{FFFD}') {
        panic!(
            "{} Failed to convert OsStr to String. Please use utf8 chars to name the target",
            mark::fail()
        );
    } else {
        // 如果没有 �，可能是平台差异，允许返回
        lossy.into_owned()
    }
}
