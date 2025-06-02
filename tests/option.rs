use std::collections::HashMap;

#[test]
fn test() {
    // let t = stringify!(all.then_some(1));
    // println!("t={}", t);
    let a = Some(Some(1));
    println!("{:?}", a.iter().flatten());
    println!(
        "{:?}",
        some_to_map(vec![("key1", &Some(true)), ("key2", &None)])
    );
}

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
