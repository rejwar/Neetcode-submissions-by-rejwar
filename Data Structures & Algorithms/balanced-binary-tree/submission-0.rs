use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check_height(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(n) = node {
                let n_borrow = n.borrow();
                
                let left = check_height(n_borrow.left.as_ref());
                if left == -1 {
                    return -1;
                }
                
                let right = check_height(n_borrow.right.as_ref());
                if right == -1 {
                    return -1;
                }
                
                if (left - right).abs() > 1 {
                    return -1;
                }
                
                std::cmp::max(left, right) + 1
            } else {
                0
            }
        }
        
        check_height(root.as_ref()) != -1
    }
}