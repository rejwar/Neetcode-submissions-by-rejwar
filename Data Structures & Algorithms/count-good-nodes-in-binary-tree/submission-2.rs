use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, i32::MIN)
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut max_val: i32) -> i32 {
        if let Some(n) = node {
            let n_ref = n.borrow();
            let mut res = 0;
            
            if n_ref.val >= max_val {
                res = 1;
                max_val = n_ref.val;
            }
            
            res + Self::dfs(&n_ref.left, max_val) + Self::dfs(&n_ref.right, max_val)
        } else {
            0
        }
    }
}