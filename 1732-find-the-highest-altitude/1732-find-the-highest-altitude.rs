use std::cmp;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut cur = 0;
        for n in gain {
            cur += n;
            max = cmp::max(max, cur);
        }
        
        max
    }
}