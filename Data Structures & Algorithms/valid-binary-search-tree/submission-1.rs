use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn validate(node: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
            if let Some(n) = node {
                let val = n.borrow().val as i64;
                
                if val <= min || val >= max {
                    return false;
                }
                
                return validate(&n.borrow().left, min, val) && 
                       validate(&n.borrow().right, val, max);
            }
            true
        }
        
        validate(&root, i64::MIN, i64::MAX)
    }
}