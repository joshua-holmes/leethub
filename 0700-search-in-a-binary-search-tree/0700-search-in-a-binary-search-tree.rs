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
impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = root;
        while let Some(n) = node {
            let node_val = n.try_borrow().unwrap().val;
            if node_val == val {
                return Some(n);
            } else if node_val > val {
                node = n.try_borrow().unwrap().left.clone();
            } else {
                node = n.try_borrow().unwrap().right.clone();
            }
        }
        
        None
    }
}