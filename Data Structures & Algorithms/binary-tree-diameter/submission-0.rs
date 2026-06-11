use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diameter = 0;
        Self::dfs(&root, &mut max_diameter);
        max_diameter
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
        let n = match node {
            None => return 0,
            Some(n) => n.borrow(),
        };

        let left_h  = Self::dfs(&n.left,  max_diameter);
        let right_h = Self::dfs(&n.right, max_diameter);

        // এই node দিয়ে যাওয়া path = left_h + right_h edges
        *max_diameter = (*max_diameter).max(left_h + right_h);

        // parent-কে height দিই: 1 + লম্বা দিক
        1 + left_h.max(right_h)
    }
}