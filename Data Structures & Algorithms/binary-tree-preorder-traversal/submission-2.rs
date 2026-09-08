use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();

        if let Some(node) = root {
            stack.push(node);
        }

        while let Some(node) = stack.pop() {
            let node_ref = node.borrow();
            result.push(node_ref.val);
            
            if let Some(right) = &node_ref.right {
                stack.push(Rc::clone(right));
            }
            if let Some(left) = &node_ref.left {
                stack.push(Rc::clone(left));
            }
        }

        result
    }
}