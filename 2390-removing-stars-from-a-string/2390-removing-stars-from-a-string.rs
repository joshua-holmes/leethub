impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut answer = String::new();
        
        for b in s.as_bytes() {
            if *b as char == '*' {
                answer.pop();
            } else {
                answer.push(*b as char);
            }
        }
        
        answer
    }
}