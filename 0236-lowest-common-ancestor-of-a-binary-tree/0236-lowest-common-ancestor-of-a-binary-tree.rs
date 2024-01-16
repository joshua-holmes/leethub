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
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root.unwrap();
        let mut lca = None;
        let p_val = p.unwrap().try_borrow().unwrap().val;
        let q_val = q.unwrap().try_borrow().unwrap().val;
        
        Self::dfs(root.clone(), &mut lca, p_val, q_val);
        
        lca
    }
    
    fn dfs(node_rc: Rc<RefCell<TreeNode>>, lca: &mut Option<Rc<RefCell<TreeNode>>>, p_val: i32, q_val: i32) -> (bool, bool) {
        let node = node_rc.try_borrow().unwrap();
        
        let mut has_p = node.val == p_val;
        let mut has_q = node.val == q_val;
        if let Some(n) = node.left.as_ref() {
            let results = Self::dfs(n.clone(), lca, p_val, q_val);
            has_p = has_p || results.0;
            has_q = has_q || results.1;
        };
        if let Some(n) = node.right.as_ref() {
            let results = Self::dfs(n.clone(), lca, p_val, q_val);
            has_p = has_p || results.0;
            has_q = has_q || results.1;
        }
        
        if lca.is_none() && has_p && has_q {
            *lca = Some(node_rc.clone());
        }
        
        (has_p, has_q)
    }
}