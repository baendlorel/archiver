use serde::{Serialize, de::DeserializeOwned};

// HACK 这段有问题，可以参考重写
/// 剥离JSON字符串中的注释（支持 // 和 /* */ 风格的注释）
fn strip_json_comments(json_str: &str) -> String {
    let mut result = String::new();
    let mut chars = json_str.chars().peekable();
    let mut in_string = false;
    let mut escape_next = false;

    while let Some(ch) = chars.next() {
        if escape_next {
            result.push(ch);
            escape_next = false;
            continue;
        }

        match ch {
            '"' => {
                in_string = !in_string;
                result.push(ch);
            }
            '\\' if in_string => {
                escape_next = true;
                result.push(ch);
            }
            '/' if !in_string => {
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '/' => {
                            // 单行注释，跳过到行末
                            chars.next(); // 消费第二个 '/'
                            while let Some(ch) = chars.next() {
                                if ch == '\n' {
                                    result.push(ch); // 保留换行符
                                    break;
                                }
                            }
                        }
                        '*' => {
                            // 多行注释，跳过到 */
                            chars.next(); // 消费 '*'
                            let mut found_end = false;
                            while let Some(ch) = chars.next() {
                                if ch == '*' {
                                    if let Some(&'/') = chars.peek() {
                                        chars.next(); // 消费 '/'
                                        found_end = true;
                                        break;
                                    }
                                }
                            }
                            if !found_end {
                                // 如果多行注释没有正确结束，保留原始内容
                                result.push('/');
                                result.push('*');
                            }
                        }
                        _ => result.push(ch),
                    }
                } else {
                    result.push(ch);
                }
            }
            _ => result.push(ch),
        }
    }

    result
}

pub trait SerdeJson: Sized + Serialize + DeserializeOwned {
    /// 从json字符串转换为对象
    fn from_json_string(s: &str) -> serde_json::Result<Self> {
        // 先剥离s中的注释
        let stripped = strip_json_comments(s);
        serde_json::from_str(&stripped)
    }

    /// 把json转换为好看格式的字符串
    fn to_formatted_string(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// 把json转换为一行的字符串
    fn to_json_line(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

// blanket impl for all Serialize + DeserializeOwned types
impl<T> SerdeJson for T where T: Sized + Serialize + DeserializeOwned {}
