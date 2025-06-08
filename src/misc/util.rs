/// 获取一个不异常大的最大值
/// - 使用3-σ原则
/// - 如果最大值没有大得很离谱，那么依然使用最大值
pub fn nsigma(arr: &[usize]) -> usize {
    if arr.is_empty() {
        println!("Array is empty, returning 0");
        return 0;
    }

    // 计算平均数μ和标准差σ
    let mu = arr.iter().sum::<usize>() as f32 / arr.len() as f32;
    let sigma =
        (arr.iter().map(|x| (*x as f32 - mu).powi(2)).sum::<f32>() / arr.len() as f32).sqrt();
    let supremum = mu + 1.0 * sigma;
    let mut pmax = 0;
    for &x in arr {
        if x > pmax && x <= supremum as usize {
            pmax = x;
        }
    }

    println!(
        "true max:{}, proper max:{} , sigma:{}",
        arr.iter().max().unwrap(),
        pmax,
        sigma
    );

    pmax
}

/// 整数均匀分拆，将n拆分成k份
/// - 每份的平均值约等于n/k
/// - 前面小后面大
///
/// e.g. 7拆成3份会变成[2, 2, 3]
pub fn int_partition(n: usize, k: usize) -> Vec<usize> {
    let avg = n / k;
    let kavg = k * avg;
    let delta = n - kavg;
    let mut result = vec![avg; k];

    for i in k - delta..k {
        result[i] += 1;
    }

    result
}

pub fn chunkify(s: &str, chunk_size: usize) -> Vec<String> {
    if chunk_size == 0 {
        panic!("Chunk size cannot be zero");
    }

    let mut chunks = Vec::new();
    let mut count = 0;
    let mut chars = s.chars().peekable();
    let mut csi_list: Vec<String> = vec![];

    let mut chunk = String::new();
    let mut chunk_csi = String::new();
    while let Some(c) = chars.next() {
        // 如果是CSI序列的开始，那么看一下下面是不是`[`，是的话要一次性扫描到控制序列结尾
        if c == '\x1b' {
            if let Some(c_next) = chars.peek() {
                if *c_next == '[' {
                    chunk_csi.push(c);
                    while let Some(c1) = chars.next() {
                        chunk_csi.push(c1);
                        if c1 == 'm' {
                            csi_list.push(chunk_csi.clone());
                            chunk_csi.clear();
                            break; // 结束CSI序列
                        }
                    }
                }
            }
        } else if c.is_control() {
            chunk.push(c);
        } else {
            chunk.push(c);
            count += 1;
        }
        if count >= chunk_size {
            // 如果当前chunk已经满了，那么将其添加到chunks中
            chunks.push(chunk.clone());
            chunk.clear();
            count = 0;
        }
    }

    chunks
}
