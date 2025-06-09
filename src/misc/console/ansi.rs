use vte::{Parser, Perform};

pub struct Stripper {
    pub plain_str: String,
    pub csi: String,
}

impl Stripper {
    fn parse(s: impl AsRef<str>) -> Self {
        let input = s.as_ref();
        let mut parser: Parser<1024> = Parser::new();
        let mut performer = Stripper {
            plain_str: String::new(),
            csi: String::new(),
        };
        let bytes = input.as_bytes();
        for &b in bytes {
            parser.advance(&mut performer, &[b]);
        }
        performer
    }
}

impl Perform for Stripper {
    fn print(&mut self, c: char) {
        self.plain_str.push(c);
    }

    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        intermediates: &[u8],
        _ignore: bool,
        final_byte: char,
    ) {
        // 构造CSI序列字符串，例如 "\x1b[31m"
        let mut csi = String::from("\x1b[");
        // 拼接参数
        let mut first = true;
        for param in params.iter() {
            if !first {
                csi.push(';');
            }
            first = false;
            let values: Vec<String> = param.iter().map(|v| v.to_string()).collect();
            csi.push_str(&values.join(","));
        }
        // 拼接intermediates
        for &b in intermediates {
            csi.push(b as char);
        }
        // 拼接final_byte
        csi.push(final_byte);
        self.csi.push_str(&csi);
    }

    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _final_byte: u8) {
        // print!("<<ESC>>"); // 控制序列
    }
}

#[test]
fn a() {
    let input = "\x1b[31mRed Tex\n\t\rt\x1b[0m Normal";
    let result = Stripper::parse(input);

    println!(
        "plain: {},  csi:{}++++",
        result.plain_str,
        result.csi.escape_default()
    );
}
