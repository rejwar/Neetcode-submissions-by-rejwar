use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::inorder(&root, &mut result);
        result
    }

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n_ref = n.borrow();
            Self::inorder(&n_ref.left, result);
            result.push(n_ref.val);
            Self::inorder(&n_ref.right, result);
        }
    }
}