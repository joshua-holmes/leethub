use std::cmp;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut my_k = k;
        
        while right < nums.len() {
            if (nums[right] == 0) {
                my_k -= 1;
            }
            if (my_k < 0) {
                if nums[left] == 0 {
                    my_k += 1;
                }
                left += 1;
            }
            right += 1;
        }
        
        (right - left) as i32
    }
}
