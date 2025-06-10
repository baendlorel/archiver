use vte::{Parser, Perform};

const CSI_END: &str = "\x1b[0m"; // ANSI控制序列结束符

/// 为带样式拆分字符串所用
struct Chunkifier {
    chunks: Vec<String>,
    chunk: String,
    ansi_chunk: String,
    chunk_size: usize,
    count: usize,
}

impl Perform for Chunkifier {
    fn print(&mut self, c: char) {
        self.chunk.push(c);
        self.count += 1;
        if self.count >= self.chunk_size {
            self.attach();
        }
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
        self.chunk.push_str(&csi);
        if csi == CSI_END {
            self.ansi_chunk.clear();
        } else {
            self.ansi_chunk.push_str(&csi);
        }
    }

    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _final_byte: u8) {
        // print!("<<ESC>>"); // 控制序列
    }
}

impl Chunkifier {
    fn attach(&mut self) {
        // 当chunk大小达到指定值时，将其添加到chunks中并重置chunk
        if self.ansi_chunk.is_empty() {
            self.chunks.push(self.chunk.clone());
        } else {
            self.chunks.push(format!("{}{}", self.chunk, CSI_END));
        }
        self.chunk.clear();
        self.chunk.push_str(&self.ansi_chunk);
        self.count = 0;
    }
}

struct Stripper {
    plain_str: String,
}

impl Perform for Stripper {
    fn print(&mut self, c: char) {
        self.plain_str.push(c);
    }
    fn execute(&mut self, byte: u8) {
        // 只要是有效的Unicode控制字符，都保留
        if let Some(c) = char::from_u32(byte as u32) {
            self.plain_str.push(c);
        }
    }
}

pub fn chunkify(s: impl AsRef<str>, chunk_size: usize) -> Vec<String> {
    let input = s.as_ref();
    let mut parser: Parser<1024> = Parser::new();
    let mut performer = Chunkifier {
        chunks: vec![],
        chunk: String::new(),
        ansi_chunk: String::new(),
        chunk_size,
        count: 0,
    };
    let bytes = input.as_bytes();
    for &b in bytes {
        parser.advance(&mut performer, &[b]);
    }
    // 把最后一小段接上去，严谨起见如果为空就不要接上了
    if !performer.chunk.is_empty() {
        performer.attach();
    }

    performer.chunks
}

pub fn strip(s: impl AsRef<str>) -> String {
    let mut parser: Parser<1024> = Parser::new();
    let mut performer = Stripper {
        plain_str: String::new(),
    };
    let bytes = s.as_ref().as_bytes();
    for &b in bytes {
        parser.advance(&mut performer, &[b]);
    }
    performer.plain_str
}

#[test]
fn 测试() {
    let input = "\x1b[31mRed Tex\n\t\rt\x1b[0m Normal";

    let stripped = strip(input);
    assert!(
        stripped == "Red Tex\n\t\rt Normal",
        "Stripped: {}",
        stripped
    );

    let result = chunkify(input, 5);

    result.iter().for_each(|s| println!("{}", s));
    println!("-------");
    result
        .iter()
        .for_each(|s| println!("{}", s.escape_default()));
}
