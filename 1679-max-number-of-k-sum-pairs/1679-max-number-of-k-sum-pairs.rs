use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let mut h_map = HashMap::new();
        let mut count = 0;
        
        for n in nums {
            let target = k - n;
            if let Some(target_count) = h_map.get_mut(&target) {
                if *target_count > 0 {
                    count += 1;
                    if *target_count == 1 {
                        h_map.remove(&target);
                    } else {
                        *target_count -= 1;
                    }
                }
            } else {
                h_map.entry(n).and_modify(|t_count| *t_count += 1).or_insert(1);
            }
        }
        
        count
    }
}