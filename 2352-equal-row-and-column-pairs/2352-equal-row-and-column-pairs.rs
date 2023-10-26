impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let len = grid.len();
        
        let mut cols: Vec<Vec<i32>> = vec![Vec::with_capacity(len); len];
        for row in &grid {
            for i in 0..len {
                cols[i].push(row[i]);
            }
        }
        
        for i in 0..len {
            for j in 0..len {
                if cols[i] == grid[j] {
                    count += 1;
                }
            }
        }
        
        count
    }
}