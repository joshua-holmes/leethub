impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let chars = s.as_bytes();
        let mut new_string = s.clone();
        let mut new_string = unsafe { new_string.as_bytes_mut() };
        
        let mut left = 0;
        let mut right = s.len() - 1;
        let vowels = "aeiouAEIOU";
        
        while left < right {
            let l_is_vowel = vowels.contains(chars[left] as char);
            let r_is_vowel = vowels.contains(chars[right] as char);
            if !l_is_vowel { left += 1; }
            if !r_is_vowel { right -= 1; }
            if r_is_vowel && l_is_vowel {
                new_string[left] = chars[right];
                new_string[right] = chars[left];
                left += 1;
                right -= 1;
            }
        }
        
        String::from_utf8(new_string.to_vec()).unwrap()
    }
}
