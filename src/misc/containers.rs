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

#[macro_export]
macro_rules! opt_map {
    () => { None };
    ( $( $e:expr ),* $(,)? ) => {{
        use crate::traits::{GetType, EnsureOptionExt};
        let mut m = std::collections::HashMap::new();
        $(
            let key = stringify!($e);
            let tp = $e.get_type();
            let optioned = $e.ensure_option();
            // 这里要创建的是纯string->string的map
            let value = match tp {
                crate::traits::VarType::Bool => {
                    // 如果是布尔值，则直接使用true或false
                    m.insert(key.to_string(), if $e { "b1".to_string() } else { "b0".to_string() });
                }
                crate::traits::VarType::String => {
                    // 如果是字符串，则转换为String
                    m.insert(key.to_string(), format!("s{}", $e));
                }
                crate::traits::VarType::OptionString => {
                    if let Some(v) = optioned {
                        // 如果是Option类型，且有值，则转换为String
                        m.insert(key.to_string(), format!("s{}",v));
                    }
                }
                crate::traits::VarType::OptionInt => {
                    if let Some(v) = optioned {
                        // 如果是Option类型，且有值，则转换为String
                        m.insert(key.to_string(), format!("i{}",v));
                    }
                }
                _ => { }
            };
        )*

        if m.len() == 0 {
            None
        } else {
            Some(m)
        }
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

// /// 从一个(key,Option<value>))元组的数组中提取value是Some的值组成HashMap
// /// - 如果一个Some都没有，那么返回None
// /// - 目前专门用在Operation构建方法的opts参数构建
// pub fn some_to_map(arr: Vec<(&str, &Option<Value>)>) -> Option<HashMap<String, Value>> {
//     let mut result: HashMap<String, Value> = HashMap::new();
//     for (key, some_value) in arr {
//         if let Some(value) = some_value {
//             result.insert(key.to_string(), value.clone());
//         }
//     }
//     if result.len() == 0 {
//         None
//     } else {
//         Some(result)
//     }
// }

pub fn some_to_map<T>(arr: Vec<(&str, &Option<T>)>) -> Option<HashMap<String, String>>
where
    T: std::fmt::Display,
{
    let mut result: HashMap<String, String> = HashMap::new();
    for (k, o) in arr {
        if let Some(value) = o {
            result.insert(k.to_string(), format!("{}", value));
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
