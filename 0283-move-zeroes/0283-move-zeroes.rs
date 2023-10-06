impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut snowBallSize = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                snowBallSize += 1;
            } else if snowBallSize > 0 {
                nums[i - snowBallSize] = nums[i];
                nums[i] = 0;
            }
        }
    }
}