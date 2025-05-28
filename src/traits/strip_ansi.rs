use strip_ansi_escapes::strip_str;

pub trait StripAnsi {
    fn strip_ansi(&self) -> String;
    fn true_len(&self) -> usize {
        self.strip_ansi().len()
    }
}

// 这样可以支持: &str, String, &String, Cow<str> 等所有字符串类型
impl<T> StripAnsi for T
where
    T: AsRef<str>,
{
    fn strip_ansi(&self) -> String {
        strip_str(self.as_ref())
    }
}
