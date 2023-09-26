use std::cmp;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut answer = String::new();
        let longest_word = cmp::max(word1.len(), word2.len());
        
        for i in 0..longest_word {
            if i < word1.len() { answer.push(word1.as_bytes()[i] as char); }
            if i < word2.len() { answer.push(word2.as_bytes()[i] as char); }
        }
        
        answer
    }
}