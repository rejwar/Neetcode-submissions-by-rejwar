use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>, 
        p: Option<Rc<RefCell<TreeNode>>>, 
        q: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut current = root;
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        while let Some(node) = current.clone() {
            let val = node.borrow().val;
            
            if p_val < val && q_val < val {
                current = node.borrow().left.clone();
            } else if p_val > val && q_val > val {
                current = node.borrow().right.clone();
            } else {
                return Some(node);
            }
        }
        
        None
    }
}