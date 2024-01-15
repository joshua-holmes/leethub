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
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut cache = HashMap::new();
        cache.insert(0, 1);
        let mut result = 0;
        
        if root.is_none() {
            return 0;
        }
        
        Self::dfs(root.unwrap().clone(), target_sum, 0, &mut cache, &mut result);
        
        result
    }
    
    pub fn dfs(node: Rc<RefCell<TreeNode>>, target_sum: i32, cur_sum: i64, cache: &mut HashMap<i64, i32>, result: &mut i32) {
        let node = node.try_borrow().unwrap();
        
        let cur_sum = cur_sum + node.val as i64;
        let missing_val = cur_sum - target_sum as i64;
        
        let num_of_target_paths = *cache.get(&missing_val).unwrap_or(&0);
        *result += num_of_target_paths;
        
        cache.entry(cur_sum).and_modify(|v| *v += 1).or_insert(1);
        
        if let Some(n) = node.right.as_ref() {
            Self::dfs(n.clone(), target_sum, cur_sum, cache, result);
        }
        if let Some(n) = node.left.as_ref() {
            Self::dfs(n.clone(), target_sum, cur_sum, cache, result);
        }
        
        cache.entry(cur_sum).and_modify(|v| *v -= 1);
    }
}