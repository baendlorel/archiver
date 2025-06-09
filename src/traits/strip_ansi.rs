use crate::misc::console::ansi;

pub trait StripAnsi: AsRef<str> {
    fn omit_skip_ansi(&self, len: usize) -> String;

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
impl<T> StripAnsi for T
where
    T: AsRef<str>,
{
    fn omit_skip_ansi(&self, len: usize) -> String {
        let s = self.as_ref();
        let stripped = ansi::strip(s);
        let mut index: usize = 0;
        let mut result = String::new();
        let mut has_controll = false;
        let chars: Vec<char> = s.chars().collect();
        let stripped_chars = stripped.chars().collect::<Vec<char>>();

        for c in chars {
            result.push(c);
            if c == stripped_chars[index] {
                index += 1;
                if index >= len {
                    break;
                }
            } else {
                has_controll = true;
            }
            // & 保留这个注释，当以后出现len对不上字数时再使用
            // println!("{}/{} - {:?}", index, len, result);
        }

        // 如果检测到控制字符，那么添加一个闭合样式字符
        if has_controll && !result.ends_with("\x1b[0m") {
            result.push_str("\x1b[0m");
        }
        result
    }
}
