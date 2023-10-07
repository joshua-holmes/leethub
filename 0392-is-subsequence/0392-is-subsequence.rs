impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }
        
        let mut s_chars = s.chars();
        let mut t_chars = t.chars();
        let mut cur_s = s_chars.next();
        let mut cur_t = t_chars.next();
        loop {
            match cur_s {
                Some(s) => {
                    match cur_t {
                        Some(t) => {
                            if t == s {
                                cur_s = s_chars.next();
                            }
                            cur_t = t_chars.next();
                        }
                        None => return false
                    }
                }
                None => return true
            }
        }
    }
}