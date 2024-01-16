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
use std::cmp;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut max_len = 0;
        let mut stack = vec![(root.unwrap(), 0, false)]; // (node, cur_len, is_left)

        while let Some((node, cur_len, is_left)) = stack.pop() {
            let node = node.try_borrow().unwrap();
            max_len = cmp::max(max_len, cur_len);

            if let Some(n) = node.right.as_ref() {
                let new_len = if cur_len == 0 || !is_left {
                    1
                } else {
                    cur_len + 1
                };
                stack.push((n.clone(), new_len, false));
            }

            if let Some(n) = node.left.as_ref() {
                let new_len = if cur_len == 0 || is_left {
                    1
                } else {
                    cur_len + 1
                };
                stack.push((n.clone(), new_len, true));
            }
        }

        max_len
    }
}