use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut found_null = false;

        while let Some(node_opt) = queue.pop_front() {
            match node_opt {
                Some(node) => {
                    if found_null {
                        return false;
                    }
                    let node_ref = node.borrow();
                    queue.push_back(node_ref.left.clone());
                    queue.push_back(node_ref.right.clone());
                }
                None => {
                    found_null = true;
                }
            }
        }
        true
    }
}