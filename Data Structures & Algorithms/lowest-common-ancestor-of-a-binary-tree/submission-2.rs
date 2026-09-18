use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                if Some(Rc::clone(&node)) == p || Some(Rc::clone(&node)) == q {
                    return Some(node);
                }
                
                let left = Self::lowest_common_ancestor(
                    node.borrow().left.clone(), 
                    p.clone(), 
                    q.clone()
                );
                let right = Self::lowest_common_ancestor(
                    node.borrow().right.clone(), 
                    p.clone(), 
                    q.clone()
                );
                
                match (left, right) {
                    (Some(_), Some(_)) => Some(node),
                    (Some(l), None) => Some(l),
                    (None, Some(r)) => Some(r),
                    (None, None) => None,
                }
            }
        }
    }
}