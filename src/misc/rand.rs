pub fn string(len: usize) -> String {
    // 可自定义字符集
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let charset_len = CHARSET.len();

    // 用系统时间做种子（不安全但足够临时用）
    let mut seed = chrono::Local::now().timestamp_millis() as usize;

    let mut s = String::with_capacity(len);
    for i in 0..len {
        // 简单线性同余生成伪随机数
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223 + i);
        let idx = seed % charset_len;
        s.push(CHARSET[idx] as char);
    }
    s
}
