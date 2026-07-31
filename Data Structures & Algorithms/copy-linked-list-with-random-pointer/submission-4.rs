use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut map = HashMap::new();
        let mut curr = head.clone();

        while let Some(node) = curr {
            map.insert(
                Rc::as_ptr(&node) as usize,
                Rc::new(RefCell::new(Node {
                    val: node.borrow().val,
                    next: None,
                    random: None,
                })),
            );
            curr = node.borrow().next.clone();
        }

        curr = head.clone();
        while let Some(node) = curr {
            let copy = map.get(&(Rc::as_ptr(&node) as usize)).unwrap().clone();
            
            copy.borrow_mut().next = node
                .borrow()
                .next
                .as_ref()
                .map(|n| map.get(&(Rc::as_ptr(n) as usize)).unwrap().clone());
                
            copy.borrow_mut().random = node
                .borrow()
                .random
                .as_ref()
                .map(|n| map.get(&(Rc::as_ptr(n) as usize)).unwrap().clone());
                
            curr = node.borrow().next.clone();
        }

        head.map(|node| map.get(&(Rc::as_ptr(&node) as usize)).unwrap().clone())
    }
}