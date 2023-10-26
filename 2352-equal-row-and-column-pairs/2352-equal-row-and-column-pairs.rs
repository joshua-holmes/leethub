use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let len = grid.len();
        
        let mut rows = HashMap::with_capacity(len);
        for row in &grid {
            *rows.entry(row).or_insert(0) += 1;
        }
        
        for i in 0..len {
            let mut col = Vec::with_capacity(len);
            for j in 0..len {
                col.push(grid[j][i]);
            }
            
            if let Some(value) = rows.get(&col) {
                count += value;
            }
        }
        
        count
    }
}