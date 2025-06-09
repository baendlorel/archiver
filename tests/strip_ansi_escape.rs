use vte::{Parser, Perform};

struct MyParser;

impl Perform for MyParser {
    fn print(&mut self, c: char) {
        print!("{}", c); // 普通字符
    }

    fn execute(&mut self, byte: u8) {
        print!("(exec:{:02x})", byte); // 控制符（如换行 \n = 0x0A）
    }

    fn csi_dispatch(
        &mut self,
        _params: &vte::Params,
        _intermediates: &[u8],
        _ignore: bool,
        _final_byte: char,
    ) {
        print!(
            "<<CSI {:?} | {:?} | {} | {}>>",
            _params, _intermediates, _ignore, _final_byte
        ); // 控制序列
    }

    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _final_byte: u8) {
        print!("<<ESC>>"); // 控制序列
    }
}
#[test]
fn a() {
    let input = "\x1b[31mRed Tex\n\t\rt\x1b[0m Normal";
    let mut parser: Parser<1024> = Parser::new();
    let mut performer = MyParser;

    println!("Input: {}", input);
    let bytes = input.as_bytes();
    for &b in bytes {
        parser.advance(&mut performer, &[b]);
    }
}
