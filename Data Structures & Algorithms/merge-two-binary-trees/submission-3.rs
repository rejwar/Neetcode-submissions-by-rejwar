use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            // If both nodes exist, add the values and recurse on children
            (Some(node1), Some(node2)) => {
                let mut n1 = node1.borrow_mut();
                let n2 = node2.borrow();
                
                n1.val += n2.val;
                
                // Use .take() to temporarily move ownership out of n1, 
                // and .clone() to increment the reference count for n2
                n1.left = Self::merge_trees(n1.left.take(), n2.left.clone());
                n1.right = Self::merge_trees(n1.right.take(), n2.right.clone());
                
                drop(n1); // Explicitly drop the mutable borrow before returning the Rc
                Some(node1)
            }
            // If only one node exists, return it
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            // If neither exists, return None
            (None, None) => None,
        }
    }
}