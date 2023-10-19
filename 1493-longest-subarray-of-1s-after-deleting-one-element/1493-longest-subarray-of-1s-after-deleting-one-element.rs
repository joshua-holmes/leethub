impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut num_of_0s = 0;
        let mut left: i32 = 0;
        let mut max = 0;
        
        for (i, n) in nums.iter().enumerate() {
            println!("{:?} {:?}", left, i);
            if *n == 0 {
                num_of_0s += 1;
            }
            while num_of_0s > 1 {
                if left > -1 && nums[left as usize] == 0 {
                    num_of_0s -= 1;
                }
                left += 1;
            }
            
            
            println!("{:?}", num_of_0s);
            if num_of_0s <= 1 {
                let count = i as i32 - left;
                if count > max { max = count; }
            }
        }
        
        max
    }
}