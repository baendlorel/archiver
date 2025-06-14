pub trait EnsureOption {
    type Output;
    fn ensure_option(self) -> Self::Output;
}

// 为 Option<T> 实现 EnsureOption，返回自身（避免双重包装）
impl<T> EnsureOption for Option<T> {
    type Output = Self;
    fn ensure_option(self) -> Self::Output {
        self
    }
}

// 在对Vec使用ensure_option时，如果Vec为空，则返回None，否则返回Some(Vec)
impl<T> EnsureOption for Vec<T> {
    type Output = Option<Self>;
    fn ensure_option(self) -> Self::Output {
        if self.is_empty() { None } else { Some(self) }
    }
}

// 用于批量实现 GetType trait 的宏
macro_rules! impl_for {
    ($($t:ty),+) => {
        $(
            impl EnsureOption for $t {
                type Output = Option<$t>;
                fn ensure_option(self) -> Self::Output {
                    Some(self)
                }
            }
        )+
    };
}
impl_for!(String, bool, u32);

/// ensure_option对'static &str的情况直接包装为Some(String)
impl EnsureOption for &str {
    type Output = Option<String>;
    fn ensure_option(self) -> Self::Output {
        Some(self.to_string())
    }
}
