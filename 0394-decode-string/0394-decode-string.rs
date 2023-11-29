impl Solution {
    pub fn decode_string(s: String) -> String {
        decode(s.as_str(), 1)
    }
}

fn decode(s: &str, num: usize) -> String {
    let mut result = String::with_capacity(s.len());
    let mut num_str = String::new();

    let mut i = 0;
    while i < s.len() {
        let c = s.as_bytes()[i] as char;
        if c.is_digit(10) {
            num_str.push(c);
        } else if c == '[' {
            let i_of_close_brack = find_closing_bracket_index(&s[i..]).unwrap() + i;
            let num: usize = num_str.parse().unwrap();
            result += &decode(&s[(i + 1)..i_of_close_brack], num);
            
            i = i_of_close_brack;
            num_str.clear();
        } else {
            result.push(c);
        }
        i += 1;
    }
    
    let mut multiplied_result = String::with_capacity(result.len() * num);
    for _ in 0..num {
        multiplied_result += &result;
    }
    multiplied_result
}

fn find_closing_bracket_index(s: &str) -> Option<usize> {
    let mut open = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '[' {
            open += 1;
        } else if c == ']' {
            open -= 1;
            if open == 0 {
                return Some(i);
            }
        }
    }
    None
}