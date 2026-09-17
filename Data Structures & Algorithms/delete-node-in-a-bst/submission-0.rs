use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node_rc) = root.clone() {
            let mut node = node_rc.borrow_mut();
            
            if key < node.val {
                let left = node.left.take();
                node.left = Self::delete_node(left, key);
            } else if key > node.val {
                let right = node.right.take();
                node.right = Self::delete_node(right, key);
            } else {
                if node.left.is_none() {
                    return node.right.take();
                }
                if node.right.is_none() {
                    return node.left.take();
                }
                
                let mut curr = node.right.clone().unwrap();
                loop {
                    let next = curr.borrow().left.clone();
                    if let Some(next_node) = next {
                        curr = next_node;
                    } else {
                        break;
                    }
                }
                
                let min_val = curr.borrow().val;
                node.val = min_val;
                
                let right = node.right.take();
                node.right = Self::delete_node(right, min_val);
            }
            Some(node_rc.clone())
        } else {
            None
        }
    }
}