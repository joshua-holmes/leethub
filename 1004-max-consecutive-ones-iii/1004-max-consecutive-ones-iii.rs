use std::cmp;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut max = 0;
        let mut cur_1_count = 0;
        let mut left = 0;
        let mut right = 0;
        let mut my_k = k;
        
        while right < nums.len() {
            if nums[right] == 1 || my_k > 0 {
                // move `right`

                if nums[right] == 0 {
                    my_k -= 1;
                }

                cur_1_count += 1;
                max = cmp::max(max, cur_1_count);
                right += 1;
            } else {
                // move `left`

                if nums[left] == 1 || my_k < k  {
                    if nums[left] == 0 {
                        my_k += 1;
                    }
                    cur_1_count -= 1;
                }
                
                if left == right {
                    right += 1;
                }
                
                left += 1;
            }
        }
        
        max
    }
}
