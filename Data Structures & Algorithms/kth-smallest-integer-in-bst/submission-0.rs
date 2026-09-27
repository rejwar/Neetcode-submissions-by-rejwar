use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = Vec::new();
        let mut current = root;
        let mut count = 0;

        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(Rc::clone(&node));
                current = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop() {
                count += 1;
                if count == k {
                    return node.borrow().val;
                }
                current = node.borrow().right.clone();
            }
        }
        
        0
    }
}