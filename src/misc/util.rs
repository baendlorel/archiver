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
