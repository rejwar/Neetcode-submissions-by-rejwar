use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            {
                let mut n = node.borrow_mut();
                if val < n.val {
                    n.left = Self::insert_into_bst(n.left.take(), val);
                } else {
                    n.right = Self::insert_into_bst(n.right.take(), val);
                }
            }
            Some(node)
        } else {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}