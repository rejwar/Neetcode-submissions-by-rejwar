use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            Some(node) => {
                let n = node.borrow();
                let mut sum = 0;
                
                if n.val >= low && n.val <= high {
                    sum += n.val;
                }
                if n.val > low {
                    sum += Self::range_sum_bst(n.left.clone(), low, high);
                }
                if n.val < high {
                    sum += Self::range_sum_bst(n.right.clone(), low, high);
                }
                
                sum
            }
            None => 0,
        }
    }
}