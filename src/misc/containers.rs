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
