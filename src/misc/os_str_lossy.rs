use std::ffi::OsStr;

pub fn force_no_loss(osstr: &OsStr) -> Result<String, ()> {
    // 先尝试严格转换
    if let Ok(s) = osstr.to_os_string().into_string() {
        return Ok(s);
    }

    // 检查 to_string_lossy 是否引入 �
    let lossy = osstr.to_string_lossy();
    if lossy.contains('\u{FFFD}') {
        Err(())
    } else {
        // 如果没有 �，可能是平台差异，允许返回
        Ok(lossy.into_owned())
    }
}
