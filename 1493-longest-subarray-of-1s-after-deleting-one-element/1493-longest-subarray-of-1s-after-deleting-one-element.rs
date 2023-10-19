use std::cmp;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut num_of_0s = 0;
        let mut left = 0;
        let mut max = 0;
        
        for (i, n) in nums.iter().enumerate() {
            if *n == 0 {
                num_of_0s += 1;
            }
            while num_of_0s > 1 {
                if nums[left] == 0 {
                    num_of_0s -= 1;
                }
                left += 1;
            }
            if num_of_0s <= 1 {
                max = cmp::max(max, (i - left) as i32);
            }
        }
        
        max
    }
}