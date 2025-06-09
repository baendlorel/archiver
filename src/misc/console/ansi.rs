use vte::{Parser, Perform};

pub struct Ansi {
    pub chunks: Vec<String>,
    pub chunk: String,
    pub ansi_chunk: String,
    pub chunk_size: usize,
    pub count: usize,
}

impl Ansi {
    pub fn chunkify(s: impl AsRef<str>, chunk_size: usize) -> Self {
        let input = s.as_ref();
        let mut parser: Parser<1024> = Parser::new();
        let mut performer = Ansi {
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
        performer
    }
}

const CSI_END: &str = "\x1b[0m"; // ANSI控制序列结束符

impl Perform for Ansi {
    fn print(&mut self, c: char) {
        self.chunk.push(c);
        self.count += 1;
        if self.count >= self.chunk_size {
            // 当chunk大小达到指定值时，将其添加到chunks中并重置chunk
            if self.ansi_chunk.is_empty() {
                self.chunks.push(self.chunk.clone());
            } else {
                self.chunks.push(format!("{}{}", self.chunk, CSI_END));
            }
            self.chunk.clear();
            self.chunk.push_str(&self.ansi_chunk);
            self.count = 0;
            // ansi_chunk也要对应进入到下一行，但是会保持
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

#[test]
fn 测试() {
    let input = "\x1b[31mRed Tex\n\t\rt\x1b[0m Normal";
    let result = Ansi::chunkify(input, 5);

    result.chunks.iter().for_each(|s| println!("{}", s));
    println!("-------");
    result
        .chunks
        .iter()
        .for_each(|s| println!("{}", s.escape_default()));
}
