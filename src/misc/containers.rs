use std::collections::HashMap;

use crate::misc::mark;

/// 快速创建一个map
/// - 用法为 map!(key1 => value1, key2 => value2, ...)
/// - 内容必须支持clone
#[macro_export]
macro_rules! map {
    () => {{
        std::collections::HashMap::new()
    }};
    ( $( $key:expr => $val:expr ),* $(,)? ) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($key, $val); )*
        m
    }};
}

/// 快速创建一个Option<HashMap<String, Opt>>
/// - 将所有传入参数确定地clone为Option<T>
/// - 若为None，则不会加入map
/// - 若map为空，则返回None。map不为空返回Some(map)
#[macro_export]
macro_rules! opt_map {
    () => { None };
    ( $( $e:expr ),* $(,)? ) => {{
        use crate::traits::{OptBuilder, EnsureOption};
        let mut m: std::collections::HashMap<String, crate::cli::Opt> = std::collections::HashMap::new();
        $(
            // 虽然会生成重复的变量名，但是没关系，rust可以重新定义同名变量
            let cloned = $e.clone();
            let key = stringify!($e);
            let optioned = cloned.ensure_option();
            if let Some(v) = optioned {
                m.insert(key.to_string(), v.to_opt());
            }
        )*

        if m.len() == 0 {
            None
        } else {
            Some(m)
        }
    }};
}

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
