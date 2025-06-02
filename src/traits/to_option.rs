/// 一个扩展 trait，提供将值确保转换为 Option<Item> 的能力。
///
/// 如果原始值已经是 Option<Item>，则保持不变。
/// 如果原始值是 Item 类型，则将其包装为 Some(Item)。
pub trait EnsureOptionExt<Item> {
    /// 将 `self` 转换为 `Option<Item>`。
    ///
    /// # 示例
    ///
    /// ```
    /// // 由于这是在 crate 内部，你需要调整 use 语句路径
    /// // use your_crate_name::traits::to_option::EnsureOptionExt;
    ///
    /// let x = 42;
    /// let opt_x: Option<i32> = x.ensure_option();
    /// assert_eq!(opt_x, Some(42));
    ///
    /// let y: Option<&str> = Some("hello");
    /// let opt_y: Option<&str> = y.ensure_option();
    /// assert_eq!(opt_y, Some("hello"));
    ///
    /// let z: Option<i32> = None;
    /// let opt_z: Option<i32> = z.ensure_option();
    /// assert_eq!(opt_z, None);
    /// ```
    fn ensure_option(self) -> Option<Item>;
}

impl<Item, Value> EnsureOptionExt<Item> for Value
where
    Value: Into<Option<Item>>,
{
    fn ensure_option(self) -> Option<Item> {
        self.into()
    }
}
