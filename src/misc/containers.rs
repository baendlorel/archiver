use crate::misc::mark;

/// 将数组去重
/// - 将消费arr
/// - 如果数组中有重复元素，则会打印信息
pub fn dedup_and_log<T>(arr: &[T]) -> Vec<T>
where
    T: Clone + Eq,
{
    let arr_len = arr.len();
    let mut v: Vec<T> = vec![];
    for a in arr {
        if !v.contains(&a) {
            v.push(a.clone()); // 这里clone比直接clone整个数组然后move要节省
        }
    }

    if v.len() != arr_len {
        println!(
            "{} {} targets given, {} after deduplication.",
            mark::info(),
            arr_len,
            v.len()
        );
    }
    v
}

/// 快速创建一个map
/// - 用法为 map!(key1 => value1, key2 => value2, ...)
/// - 内容必须支持clone
#[macro_export]
macro_rules! map {
    () => {{
        std::collections::HashMap::new()
    }};
    ( $( $key:expr => $val:expr ),* $(,)? ) => {{
        let mut map = std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }};
}
