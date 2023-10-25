use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        // words need to be the same length
        // words need to contain the same ratio of different letters
        // words need to have the same number of different letters
        
        if word1.len() != word2.len() {
            return false;
        }
        
        let mut letters1 = HashMap::new();
        let mut letters2 = HashMap::new();
        
        for b in word1.as_bytes() {
            *letters1.entry(*b).or_insert(0) += 1;
        }
        for b in word2.as_bytes() {
            *letters2.entry(*b).or_insert(0) += 1;
        }
        
        let mut values1: Vec<&i32> = letters1.values().collect();
        let mut values2: Vec<&i32> = letters2.values().collect();
        values1.sort();
        values2.sort();
        if values1 != values2 {
            return false;
        }
        
        let keys1: HashSet<&u8> = letters1.keys().collect();
        let keys2: HashSet<&u8> = letters2.keys().collect();
        if keys1 != keys2 {
            return false;
        }
        
        true
    }
}