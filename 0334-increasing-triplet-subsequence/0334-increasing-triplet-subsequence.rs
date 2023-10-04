impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
//         let mut min_num: Option<i32> = None;
//         let mut target: Option<i32> = None;

//         for n in nums {
//             if let Some(t) = target {
//                 if n >= t { return true; }
//             }
//             match min_num {
//                 Some(min) => {
//                     if n < min {
//                         min_num = Some(n);
//                     } else if n > min && n != i32::MAX {
//                         target = match target {
//                             Some(t) => Some(std::cmp::min(t, n + 1)),
//                             None => Some(n + 1)
//                         };
//                     }
//                 }
//                 None => { min_num = Some(n) }
//             }
//         }

//         false
        let (mut a, mut b) = (i32::MAX, i32::MAX);
        for n in nums {
            if n <= a { a = n }
            else if n <= b { b = n }
            else { return true; }
        }
        return false;
    }
}