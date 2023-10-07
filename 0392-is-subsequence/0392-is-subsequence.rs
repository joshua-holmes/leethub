impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }
        
        let mut s_chars = s.chars();
        let mut t_chars = t.chars();
        let mut cur_s: Option<char> = s_chars.next();
        let mut cur_t: Option<char> = t_chars.next();
        loop {
            if let Some(s) = cur_s {
                if let Some(t) = cur_t {
                    if t == s {
                        cur_s = s_chars.next();
                    }
                    cur_t = t_chars.next();
                } else {
                    return false;
                }
            } else {
                return true;
            }
        }
    }
}