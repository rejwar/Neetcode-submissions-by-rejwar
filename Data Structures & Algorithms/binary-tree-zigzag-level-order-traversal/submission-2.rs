use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        let mut left_to_right = true;

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_level = Vec::with_capacity(level_size);

            for _ in 0..level_size {
                let node = queue.pop_front().unwrap();
                let node_borrowed = node.borrow();
                
                current_level.push(node_borrowed.val);

                if let Some(left) = &node_borrowed.left {
                    queue.push_back(Rc::clone(left));
                }
                if let Some(right) = &node_borrowed.right {
                    queue.push_back(Rc::clone(right));
                }
            }

            if !left_to_right {
                current_level.reverse();
            }

            result.push(current_level);
            left_to_right = !left_to_right;
        }

        result
    }
}