use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut curr = root;
        let mut last_visited: Option<Rc<RefCell<TreeNode>>> = None;

        while curr.is_some() || !stack.is_empty() {
            while let Some(node) = curr {
                stack.push(node.clone());
                curr = node.borrow().left.clone();
            }

            let peek = stack.last().unwrap().clone();
            let right = peek.borrow().right.clone();

            if right.is_some() && last_visited.as_ref().map_or(true, |v| !Rc::ptr_eq(v, right.as_ref().unwrap())) {
                curr = right;
            } else {
                result.push(peek.borrow().val);
                last_visited = stack.pop();
            }
        }

        result
    }
}