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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut leaves = Vec::new();
        if root1.is_none() {
            return root2.is_none();
        }
        if root2.is_none() {
            return root1.is_none();
        }
        
        let mut stack = vec![root1.unwrap()];
        while let Some(node) = stack.pop() {
            let node = node.try_borrow().unwrap();
            if node.left.is_none() && node.right.is_none() {
                // is leaf
                leaves.push(node.val);
                continue;
            }
            
            if let Some(n) = node.right.as_ref() {
                stack.push(n.clone());
            }
            if let Some(n) = node.left.as_ref() {
                stack.push(n.clone());
            }
        }
        
        // empty stack but keep it the same length, then add root2 to it
        stack.clear();
        stack.push(root2.unwrap());
        let mut count = 0;
        while let Some(node) = stack.pop() {
            let node = node.try_borrow().unwrap();
            if node.left.is_none() && node.right.is_none() {
                // is leaf
                if let Some(leaf) = leaves.get(count) {
                    if *leaf != node.val {
                        return false;
                    }
                } else {
                    return false;
                }
                count += 1;
            }
            
            if let Some(n) = node.right.as_ref() {
                stack.push(n.clone());
            }
            if let Some(n) = node.left.as_ref() {
                stack.push(n.clone());
            }
        }
        
        count == leaves.len()
    }
}