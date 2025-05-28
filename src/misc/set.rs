use std::collections::HashSet;

use crate::misc::mark;

/// 将数组去重为集合
/// - 如果数组中有重复元素，则会打印信息
pub fn dedup_to_set<T>(targets: &[T]) -> HashSet<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    let set: HashSet<T> = targets.iter().cloned().collect();
    if set.len() != targets.len() {
        println!(
            "{} {} targets given, {} after deduplication.",
            mark::info(),
            targets.len(),
            set.len()
        );
    }
    set
}
