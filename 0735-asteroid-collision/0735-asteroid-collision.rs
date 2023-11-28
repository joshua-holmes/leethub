impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut new_astroids = Vec::with_capacity(asteroids.len());
        
        'main: for a in asteroids {
            if a > 0 {
                new_astroids.push(a);
                continue;
            }
            
            while let Some(new_a) = new_astroids.last() {
                if *new_a < 0 {
                    break;
                }
                
                if -a > *new_a {
                    new_astroids.pop();
                } else if -a < *new_a {
                    continue 'main;
                } else {
                    new_astroids.pop();
                    continue 'main;
                }
            }
            
            new_astroids.push(a);
        }
        
        new_astroids
    }
}