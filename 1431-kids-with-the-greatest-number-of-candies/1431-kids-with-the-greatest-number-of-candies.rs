impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candies = *candies.iter().max().unwrap();
        let mut answer = Vec::with_capacity(candies.len());
        
        for candy in candies {
            answer.push(candy + extra_candies >= max_candies);
        }
        
        answer
    }
}