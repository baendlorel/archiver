use std::io::{self, Write};

pub fn confirm(message: &str, verify_code: &str) -> bool {
    println!("{}", message);
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    input == verify_code
}
