impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut counter = 0;
        let mut prev_char = ' ';
        let mut i = 0;
        while i < chars.len() {
            let cur_char = chars[i];            
            if cur_char == prev_char && i != 0 {
                counter += 1;
            } else {
                compress_letters(chars, &mut counter, &mut i);
            }
            prev_char = cur_char;
            i += 1;
        }
        compress_letters(chars, &mut counter, &mut i);
        chars.len() as i32
    }
}

fn compress_letters(chars: &mut Vec<char>, counter: &mut usize, i: &mut usize) {
    if *counter > 1 {
        // change characters in `chars`
        let counter_str = counter.to_string();
        let counter_str = counter_str.as_bytes();
        let second_letter_i = *i - *counter + 1;
        for j in 0..counter_str.len() {
            chars[second_letter_i + j] = counter_str[j] as char;
        }
        
        // slice elements of `chars`
        let slice_start_i = second_letter_i + counter_str.len();
        let num_of_removals = (*counter - counter_str.len() - 1);
        for j in 0..num_of_removals {
            let inverse_index = num_of_removals - j + slice_start_i - 1;
            chars.remove(inverse_index);
        }
        *i -= num_of_removals
    }
    *counter = 1;
}