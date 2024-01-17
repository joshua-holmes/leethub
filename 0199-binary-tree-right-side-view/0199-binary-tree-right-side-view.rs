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
use std::collections::VecDeque;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }
        let mut queue = VecDeque::from([(root.unwrap(), 1)]); // (node_ref, node_level)
        let mut cur_level = 0;
        while let Some((node_ref, node_level)) = queue.pop_front() {
            let node = node_ref.try_borrow().unwrap();
            if cur_level != node_level {
                result.push(node.val);
                cur_level = node_level;
            }
            
            if let Some(n) = node.right.as_ref() {
                queue.push_back((n.clone(), node_level + 1));
            }
            if let Some(n) = node.left.as_ref() {
                queue.push_back((n.clone(), node_level + 1));
            }
        }
        
        result
    }
}