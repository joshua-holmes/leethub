use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len() - 1);
        let mut water = 0;
        while i < j {
            let distance = (j - i) as i32;
            let shortest_wall = cmp::min(height[i], height[j]);
            let area = distance * shortest_wall;
            water = cmp::max(water, area);
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        
        water
    }
}