use crate::misc::console::ansi;

pub trait StripAnsi: AsRef<str> {
    fn strip_ansi(&self) -> String {
        ansi::strip(self.as_ref())
    }

    fn true_len(&self) -> usize {
        let s: String = ansi::strip(self);
        match s.as_ref() {
            // & 特殊处理这些长度为3的符号
            "✓" => 1,
            "✗" => 1,
            "⚠" => 1,
            "⚑" => 1,
            _ => s.len(),
        }
    }
}

// 这样可以支持: &str, String, &String, Cow<str> 等所有字符串类型
impl<T> StripAnsi for T where T: AsRef<str> {}
