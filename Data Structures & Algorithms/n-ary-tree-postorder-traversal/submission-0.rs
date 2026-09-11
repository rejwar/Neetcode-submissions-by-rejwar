use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn postorder(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(node) = root {
            let mut stack = vec![node];
            while let Some(current) = stack.pop() {
                let current_borrow = current.borrow();
                result.push(current_borrow.val);
                for child in &current_borrow.children {
                    stack.push(Rc::clone(child));
                }
            }
            result.reverse();
        }
        result
    }
}