impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut s_bytes = s.clone();
        let mut s_bytes = unsafe { s_bytes.as_bytes_mut() };
        let mut char_indecies: Vec<usize> = Vec::new();

        for (i, b) in s.as_bytes().iter().enumerate() {
            if *b as char != '*' {
                char_indecies.push(i);
            } else if *b as char == '*' && i > 0 {
                if let Some(j) = char_indecies.pop() {
                    s_bytes[i] = 0;
                    s_bytes[j] = 0;
                }
            }
        }
        
        let mut answer = String::new();
        for b in s_bytes {
            if *b != 0 {
                answer.push(*b as char);
            }
        }
        
        answer
    }
}