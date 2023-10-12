impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut max = i32::MIN;
        
        for left in 0..(nums.len() - k as usize + 1) {
            let right = left + k as usize - 1;
            let sum = nums[left..=right].iter().sum();
            if sum > max {
                max = sum;
            }
        }
        
        max as f64 / k as f64
    }
}