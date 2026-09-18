use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn connect(root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        if let Some(node) = &root {
            let node_ref = node.borrow();
            
            if let (Some(left), Some(right)) = (&node_ref.left, &node_ref.right) {
                left.borrow_mut().next = Some(Rc::clone(right));
                
                if let Some(next) = &node_ref.next {
                    right.borrow_mut().next = next.borrow().left.clone();
                }
            }
        }
        
        if let Some(node) = &root {
            let node_ref = node.borrow();
            Self::connect(node_ref.left.clone());
            Self::connect(node_ref.right.clone());
        }
        
        root
    }
}