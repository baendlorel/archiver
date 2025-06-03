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
