use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(n1), Some(n2)) => {
                {
                    let mut b1 = n1.borrow_mut();
                    let mut b2 = n2.borrow_mut();
                    b1.val += b2.val;
                    b1.left = Self::merge_trees(b1.left.take(), b2.left.take());
                    b1.right = Self::merge_trees(b1.right.take(), b2.right.take());
                }
                Some(n1)
            }
            (Some(n), None) | (None, Some(n)) => Some(n),
            (None, None) => None,
        }
    }
}