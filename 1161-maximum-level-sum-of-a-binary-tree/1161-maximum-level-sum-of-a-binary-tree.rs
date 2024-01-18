// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

struct LevelSum {
    level: i32,
    sum: i64,
}
impl LevelSum {
    fn update_if_larger(&mut self, comparison: &LevelSum) {
        if comparison.sum > self.sum {
            self.level = comparison.level;
            self.sum = comparison.sum;
        }
    }
    
    pub fn new() -> Self {
        Self {
            level: 0,
            sum: i64::MIN
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = LevelSum::new();
        let mut cur = LevelSum::new();
        let mut queue = VecDeque::from([(root.unwrap(), 1)]); // (node_ref, node_level)
        
        while let Some((node_ref, node_level)) = queue.pop_front() {
            let node = node_ref.try_borrow().unwrap();
            if node_level != cur.level {
                max.update_if_larger(&cur);
                cur.sum = 0;
                cur.level = node_level;
            }
            
            cur.sum += node.val as i64;
            
            if let Some(n) = node.left.as_ref() {
                queue.push_back((n.clone(), cur.level + 1));
            }
            if let Some(n) = node.right.as_ref() {
                queue.push_back((n.clone(), cur.level + 1));
            }
        }
        max.update_if_larger(&cur);
        
        max.level
    }
}