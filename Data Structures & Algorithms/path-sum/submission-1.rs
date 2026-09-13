use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node_rc) => {
                let node = node_rc.borrow();
                if node.left.is_none() && node.right.is_none() {
                    target_sum == node.val
                } else {
                    Self::has_path_sum(node.left.clone(), target_sum - node.val) ||
                    Self::has_path_sum(node.right.clone(), target_sum - node.val)
                }
            }
            None => false,
        }
    }
}