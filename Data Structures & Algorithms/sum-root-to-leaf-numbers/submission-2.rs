use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, 0)
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut current_sum: i32) -> i32 {
        if let Some(n) = node {
            let n_borrowed = n.borrow();
            // Shift the current sum left by a decimal place and add the new digit
            current_sum = current_sum * 10 + n_borrowed.val;
            
            // If it's a leaf node, return the accumulated sum for this path
            if n_borrowed.left.is_none() && n_borrowed.right.is_none() {
                return current_sum;
            }
            
            // Otherwise, recursively calculate the sum of the left and right subtrees
            return Self::dfs(&n_borrowed.left, current_sum) + 
                   Self::dfs(&n_borrowed.right, current_sum);
        }
        
        0
    }
}