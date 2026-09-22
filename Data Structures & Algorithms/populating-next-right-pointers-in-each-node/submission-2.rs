use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn connect(root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        if let Some(node) = root.clone() {
            let n = node.borrow();
            
            if let Some(left) = n.left.clone() {
                let right = n.right.clone().unwrap();
                left.borrow_mut().next = Some(right.clone());
                
                if let Some(next) = n.next.clone() {
                    right.borrow_mut().next = next.borrow().left.clone();
                }
                
                drop(n);
                
                Self::connect(Some(left));
                Self::connect(Some(right));
            }
        }
        root
    }
}