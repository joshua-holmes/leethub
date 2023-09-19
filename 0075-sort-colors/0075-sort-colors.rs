impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() == 1 { return; }
        
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut i = 0;
        while i <= right {
            println!("{} {}", i, right);
            if nums[i] == 0 {
                swap(nums, left, i);
                i += 1;
                left += 1;
            } else if nums[i] == 2 {
                swap(nums, i, right);
                if right != 0 {
                    right -= 1;
                } else {
                    break;
                }
            } else {
                i += 1;
            }
        }
    }
    
}

fn swap(nums: &mut Vec<i32>, left_i: usize, right_i: usize) {
    let left_n = nums[left_i];
    nums[left_i] = nums[right_i];
    nums[right_i] = left_n;
}
