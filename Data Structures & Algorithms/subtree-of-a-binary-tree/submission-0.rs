use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if sub_root.is_none() { return true; }
        if root.is_none() { return false; }

        if Self::is_same_tree(root.clone(), sub_root.clone()) {
            return true;
        }

        let root_ref = root.as_ref().unwrap().borrow();
        Self::is_subtree(root_ref.left.clone(), sub_root.clone()) || 
        Self::is_subtree(root_ref.right.clone(), sub_root.clone())
    }

    fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p_node), Some(q_node)) => {
                let p_ref = p_node.borrow();
                let q_ref = q_node.borrow();
                p_ref.val == q_ref.val &&
                Self::is_same_tree(p_ref.left.clone(), q_ref.left.clone()) &&
                Self::is_same_tree(p_ref.right.clone(), q_ref.right.clone())
            }
            _ => false,
        }
    }
}