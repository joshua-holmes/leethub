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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        
        let root = root.unwrap();
        let root_val = root.try_borrow().unwrap().val;
        let mut stack = vec![(root, root_val)]; // (node, max_val)
        let mut count = 0;
        while let Some((node, max_val)) = stack.pop() {
            let node = node.try_borrow().unwrap();
            let new_max_val = if node.val >= max_val {
                count += 1;
                node.val
            } else {
                max_val
            };
            
            if let Some(n) = node.right.as_ref() {
                stack.push((n.clone(), new_max_val));
            }
            if let Some(n) = node.left.as_ref() {
                stack.push((n.clone(), new_max_val));
            }
        }
        
        count
    }
}