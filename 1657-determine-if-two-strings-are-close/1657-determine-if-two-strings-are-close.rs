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
        
        let word_bytes1 = word1.as_bytes();
        let word_bytes2 = word2.as_bytes();
        
        for b in word_bytes1 {
            let letter = *b as char;
            *letters1.entry(letter).or_insert(0) += 1;
        }
        for b in word_bytes2 {
            let letter = *b as char;
            *letters2.entry(letter).or_insert(0) += 1;
        }
        
        let mut values1: Vec<&i32> = letters1.values().collect();
        let mut values2: Vec<&i32> = letters2.values().collect();
        values1.sort();
        values2.sort();
        if values1 != values2 {
            return false;
        }
        
        let keys1: HashSet<&char> = letters1.keys().collect();
        let keys2: HashSet<&char> = letters2.keys().collect();
        
        if keys1 != keys2 {
            return false;
        }
        
        true
    }
}