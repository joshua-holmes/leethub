use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut count_map = HashMap::new();
        for n in arr {
            *count_map.entry(n).or_insert(0) += 1;
        }
        
        let mut unique = HashSet::new();
        for (_, count) in count_map {
            if !unique.insert(count) {
                return false;
            }
        }
        
        true
    }
}