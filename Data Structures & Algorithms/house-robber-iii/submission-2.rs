use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (robbed, not_robbed) = Self::dfs(&root);
        robbed.max(not_robbed)
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(n) = node {
            let n_ref = n.borrow();
            let left = Self::dfs(&n_ref.left);
            let right = Self::dfs(&n_ref.right);

            let rob_this = n_ref.val + left.1 + right.1;
            let skip_this = left.0.max(left.1) + right.0.max(right.1);

            (rob_this, skip_this)
        } else {
            (0, 0)
        }
    }
}