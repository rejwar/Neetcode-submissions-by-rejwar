use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        if root.is_none() {
            return res;
        }

        let mut q = VecDeque::new();
        q.push_back(root.unwrap());

        while !q.is_empty() {
            let len = q.len();
            for i in 0..len {
                if let Some(node_rc) = q.pop_front() {
                    let node = node_rc.borrow();
                    if i == len - 1 {
                        res.push(node.val);
                    }
                    if let Some(left) = &node.left {
                        q.push_back(Rc::clone(left));
                    }
                    if let Some(right) = &node.right {
                        q.push_back(Rc::clone(right));
                    }
                }
            }
        }

        res
    }
}