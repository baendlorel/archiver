use crate::{core::log, models::error::ArchiverError};

/// Trait：为 Result 提供扩展方法
pub trait ResultExt {
    type Ok;
    type Err;

    /// 表示如果值是Ok，那么用Ok内的值调用闭包，否则记录错误日志
    fn ok_then_or_log<F>(self, f: F)
    where
        F: FnOnce(Self::Ok);

    /// 允许Err且输出error信息
    fn allow_and_display(self);
}

impl<T> ResultExt for Result<T, ArchiverError> {
    type Ok = T;
    type Err = ArchiverError;

    fn ok_then_or_log<F>(self, f: F)
    where
        F: FnOnce(T),
    {
        match self {
            Ok(val) => {
                f(val);
            }
            Err(e) => log::error(e),
        }
    }

    fn allow_and_display(self) {
        if let Err(e) = self {
            e.display();
        }
    }
}
