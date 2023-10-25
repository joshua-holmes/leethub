use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        // words need to be the same length
        // words need to contain the same ratio of different letters
        // words need to have the same letters used between words
        
        if word1.len() != word2.len() {
            return false;
        }
        
        let mut letters1 = HashSet::new();
        let mut letters2 = HashSet::new();
        let mut char_count1 = [0; 26];
        let mut char_count2 = [0; 26];
        
        for b in word1.as_bytes() {
            char_count1[(*b - 'a' as u8) as usize] += 1;
            letters1.insert(*b);
        }
        for b in word2.as_bytes() {
            char_count2[(*b - 'a' as u8) as usize] += 1;
            letters2.insert(*b);
        }

        // check that ratio of different letters is correct
        char_count1.sort();
        char_count2.sort();
        if char_count1 != char_count2 {
            return false;
        }
        
        // check that the same letters between words are used
        if letters1 != letters2 {
            return false;
        }
        
        true
    }
}