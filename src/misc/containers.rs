use serde_json::Value;
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
        let mut map = std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }};
}

#[macro_export]
macro_rules! opt_map {
    () => { None };
    ( $( $var:ident ),* $(,)? ) => {{
        let mut map = std::collections::HashMap::new();
        $(
            if let Some($val) = $var {
                map.insert(stringify!($var), $val);
            }
        )*
        map
    }};
}

/// 从一个Option的数组中提取Some的值组成新数组
/// - 返回的数组是Option的，如果原数组一个Some都没有，那么返回None
/// - 目前专门用在Operation构建方法的args参数构建
pub fn some_to_vec<T>(arr: Vec<Option<T>>) -> Option<Vec<T>> {
    let mut result: Vec<T> = vec![];
    for a in arr {
        if let Some(item) = a {
            result.push(item);
        }
    }
    if result.len() == 0 {
        None
    } else {
        Some(result)
    }
}

/// 从一个(key,Option<value>))元组的数组中提取value是Some的值组成HashMap
/// - 如果一个Some都没有，那么返回None
/// - 目前专门用在Operation构建方法的opts参数构建
pub fn some_to_map(arr: Vec<(&str, &Option<Value>)>) -> Option<HashMap<String, Value>> {
    let mut result: HashMap<String, Value> = HashMap::new();
    for (key, some_value) in arr {
        if let Some(value) = some_value {
            result.insert(key.to_string(), value.clone());
        }
    }
    if result.len() == 0 {
        None
    } else {
        Some(result)
    }
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
