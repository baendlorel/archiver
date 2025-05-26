use std::collections::HashMap;

#[test]
fn test_archive_and_restore() {
    let mut map: HashMap<String, String> = HashMap::new();
    let arr = [["a", "b"]];
    for pair in arr {
        if pair.len() == 2 {
            map.insert(pair[0].to_string(), pair[1].to_string());
        }
    }
}
