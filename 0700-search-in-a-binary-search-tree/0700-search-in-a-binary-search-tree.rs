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
        let mut cur_node = root;
        while let Some(node_rc) = cur_node {
            // check if this node is the correct node
            let node_val = node_rc.try_borrow().unwrap().val;
            if node_val == val {
                return Some(node_rc);
            }
            
            // search right and left if not
            let node = node_rc.try_borrow().unwrap();
            if node_val > val {
                cur_node = node.left.clone();
            } else {
                cur_node = node.right.clone();
            }
        }
        
        None
    }
}