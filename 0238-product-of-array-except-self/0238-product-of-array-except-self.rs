impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::with_capacity(nums.len());
        answer.push(1);
        for i in 1..nums.len() {
            answer.push(nums[i - 1] * answer[i - 1]);
        }
        let mut right = 1;
        for i in (0..nums.len()).rev() {
            answer[i] *= right;
            right *= nums[i];
        }
        answer
    }
}