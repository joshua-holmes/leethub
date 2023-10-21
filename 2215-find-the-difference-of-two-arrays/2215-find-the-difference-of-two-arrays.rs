use std::cmp;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let max_len = cmp::max(nums1.len(), nums2.len());
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();
        
        
        let mut result = vec![Vec::new(), Vec::new()];
        for n in &set1 {
            if !set2.contains(n) {
                result[0].push(*n);
            }
        }
        
        for n in &set2 {
            if !set1.contains(n) {
                result[1].push(*n);
            }
        }
        
        result
    }
}