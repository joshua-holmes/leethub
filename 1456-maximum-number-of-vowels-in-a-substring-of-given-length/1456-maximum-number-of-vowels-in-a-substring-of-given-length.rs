use std::cmp;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut max = 0;
        let string = s.as_bytes();
        
        // using array because for some reason, it is 4x faster than using HashSet
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        
        // count of vowels as window (i through j) progresses
        let mut cur_count = 0;
        
        for i in 0..(s.len() - k as usize + 1) {
            let j = i + k as usize - 1;
            
            if i == 0 {
                // first, count how many vowels from i to j
                cur_count = string[i..=j].iter().fold(0,
                    |a, e| if vowels.contains(&(*e as char)) { a + 1 } else { a }
                );
            } else {
                // every time after, only check last char of window and add to cur_count if vowel
                if vowels.contains(&(string[j] as char)) {
                    cur_count += 1;
                }
            }
            
            max = cmp::max(max, cur_count);
            
            // only check first char and decrement cur_count if vowel
            if vowels.contains(&(string[i] as char)) {
                cur_count -= 1;
            }
        }
        
        max
    }
}