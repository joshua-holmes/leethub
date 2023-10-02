impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![1; nums.len()];
        
        let mut left = 1;
        let mut right = 1;
        
        let len = nums.len();
        for i in 0..len {
            answer[i] *= left;
            answer[len - 1 - i] *= right;
            left *= nums[i];
            right *= nums[len -1 - i];
        }
        
        answer
    }
}