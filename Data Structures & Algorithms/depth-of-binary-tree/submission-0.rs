use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let n = node.borrow();
                let left_depth = Self::max_depth(n.left.clone());
                let right_depth = Self::max_depth(n.right.clone());
                
                1 + left_depth.max(right_depth)
            }
        }
    }
}