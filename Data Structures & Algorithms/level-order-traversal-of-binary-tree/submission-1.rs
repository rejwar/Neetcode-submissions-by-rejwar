use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        
        if let Some(node) = root {
            let mut queue = VecDeque::new();
            queue.push_back(node);
            
            while !queue.is_empty() {
                let level_size = queue.len();
                let mut current_level = Vec::with_capacity(level_size);
                
                for _ in 0..level_size {
                    let current_node = queue.pop_front().unwrap();
                    let node_ref = current_node.borrow();
                    
                    current_level.push(node_ref.val);
                    
                    if let Some(left) = &node_ref.left {
                        queue.push_back(Rc::clone(left));
                    }
                    if let Some(right) = &node_ref.right {
                        queue.push_back(Rc::clone(right));
                    }
                }
                
                result.push(current_level);
            }
        }
        
        result
    }
}