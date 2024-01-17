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
        let lca = None;
        let p_val = p.unwrap().try_borrow().unwrap().val;
        let q_val = q.unwrap().try_borrow().unwrap().val;
        
        // lca's ownership gets passed through the tree
        // this is an alternative option to mutable borrowing
        let lca = Self::dfs(root.clone(), lca, p_val, q_val).2;
        
        lca
    }
    
    fn dfs(
        node_rc: Rc<RefCell<TreeNode>>,
        mut lca: Option<Rc<RefCell<TreeNode>>>,
        p_val: i32,
        q_val: i32
    ) -> (
        bool, // has_p
        bool, // has_q
        Option<Rc<RefCell<TreeNode>>> // lca
    ) {
        let node = node_rc.try_borrow().unwrap();
        
        // this is `true` if this node is p or q
        let mut has_p = node.val == p_val;
        let mut has_q = node.val == q_val;
        
        // send lca through the tree to get set as `Some(_)` if it finds both p and q
        let lca = if let Some(n) = node.left.as_ref() {
            let results = Self::dfs(n.clone(), lca, p_val, q_val);
            has_p = has_p || results.0;
            has_q = has_q || results.1;
            results.2 // returning lca's ownership
        } else { lca };
        let mut lca = if let Some(n) = node.right.as_ref() {
            let results = Self::dfs(n.clone(), lca, p_val, q_val);
            has_p = has_p || results.0;
            has_q = has_q || results.1;
            results.2 // returning lca's ownership
        } else { lca };
        
        // only set lca if it is not already set
        // since this occurs after the recursion, this will get set first in the deepest part of the tree
        // since we only set lca if it is `None`, we do not overwrite lca with higher nodes after it has been written to
        if lca.is_none() && has_p && has_q {
            lca = Some(node_rc.clone());
        }
        
        (has_p, has_q, lca)
    }
}