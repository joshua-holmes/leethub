use std::collections::HashMap;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let nums = format!("{}", x);

        let mut result = 0;
        let mut is_negative = false;
        for (i, b) in nums.as_bytes().iter().enumerate() {
            if *b == 45 {
                is_negative = true;
                continue;
            }
            
            let index = if is_negative { i - 1 } else { i };
            let num = b - 48;
            let add = (10_i64.pow(index as u32)) * (num as i64);
            // println!("{:?} {:?} {:?} {:?}", i, b, num, add);
            if is_negative && result - add >= (i32::MIN as i64) {
                result -= add;
            } else if !is_negative && result + add <= (i32::MAX as i64) {
                result += add;
            } else {
                return 0;
            }
        }
        
        result as i32
    }
}