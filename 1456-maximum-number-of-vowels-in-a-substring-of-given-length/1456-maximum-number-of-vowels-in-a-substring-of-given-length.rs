use std::cmp;
use std::collections::HashSet;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut max = 0;
        let string = s.as_bytes();
        
        // store vowels in set for O(1) lookup time
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        
        // count of vowels as window (i through j) progresses
        let mut cur_count = 0;
        
        for i in 0..(s.len() - k as usize + 1) {
            let j = i + k as usize - 1;
            
            if i == 0 {
                // first, count how many vowels from i to j
                cur_count = string[i..=j].iter().fold(0,
                    |a, e| if vowels.get(&(*e as char)).is_some() { a + 1 } else { a }
                );
            } else {
                // every time after, only check last char of window and add to cur_count if vowel
                if vowels.get(&(string[j] as char)).is_some() {
                    cur_count += 1;
                }
            }
            
            max = cmp::max(max, cur_count);
            
            // only check first char and decrement cur_count if vowel
            if vowels.get(&(string[i] as char)).is_some() {
                cur_count -= 1;
            }
        }
        
        max
    }
}