use std::io::{self, Write};

/// 确认是与否
/// - 输入y、Y、yes、YES表示确认
/// - 自动在message后面加入`[y/N]>`字样
pub fn confirm(message: &str) -> bool {
    print!("{} [y/N]> ", message);
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    input == "y" || input == "Y" || input == "yes" || input == "YES"
}

/// 确认输入自定义的字符串
/// - 必须要输入和verify_code一致才算通过
/// - 不会添加任何字样
pub fn confirm_str(message: &str, verify_code: &str) -> bool {
    print!("{}", message);
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    input == verify_code
}
