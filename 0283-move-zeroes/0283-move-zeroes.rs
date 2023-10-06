impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left: usize = 0; // used for finding 0s
        let mut right: usize = 0; // used for finding non-0s
        while right < nums.len() {
            while nums[right] == 0 {
                right += 1;
                if right >= nums.len() { return; }
            }
            while nums[left] != 0 {
                left += 1;
                if left >= nums.len() { return; }
            }
            while left > right || nums[right] == 0 {
                right += 1;
                if right >= nums.len() { return; }
            }
            
            nums[left] = nums[right];
            nums[right] = 0;
        }
    }
}