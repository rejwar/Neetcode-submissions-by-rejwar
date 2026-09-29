use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, 0)
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, current_sum: i32) -> i32 {
        if let Some(n) = node {
            let n_ref = n.borrow();
            let next_sum = current_sum * 10 + n_ref.val;
            
            if n_ref.left.is_none() && n_ref.right.is_none() {
                return next_sum;
            }
            
            return Self::dfs(&n_ref.left, next_sum) + Self::dfs(&n_ref.right, next_sum);
        }
        0
    }
}