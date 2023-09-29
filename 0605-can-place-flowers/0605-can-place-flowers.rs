impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut pottable = 0;
        let mut prev = 0;
        
        for (i, pot) in flowerbed.iter().enumerate() {
            match prev {
                0 => {
                    if *pot == 0 {
                        prev = 2;
                        pottable += 1;
                        continue;
                    }
                }
                2 => {
                    if *pot == 1 {
                        pottable -= 1;
                    }
                }
                _ => {}
            }
            
            prev = *pot;
        }
        
        pottable >= n
    }
}