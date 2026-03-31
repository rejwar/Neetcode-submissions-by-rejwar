use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p_node), Some(q_node)) => {
                let p_ref = p_node.borrow();
                let q_ref = q_node.borrow();
                
                p_ref.val == q_ref.val && 
                Self::is_same_tree(p_ref.left.clone(), q_ref.left.clone()) && 
                Self::is_same_tree(p_ref.right.clone(), q_ref.right.clone())
            },
            _ => false,
        }
    }
}