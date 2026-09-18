use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn lowest_common_ancestor(
        p: Option<Rc<RefCell<Node>>>,
        q: Option<Rc<RefCell<Node>>>
    ) -> Option<Rc<RefCell<Node>>> {
        let mut a = p.clone();
        let mut b = q.clone();

        loop {
            let val_a = a.as_ref().map(|n| n.borrow().val);
            let val_b = b.as_ref().map(|n| n.borrow().val);

            if val_a == val_b && val_a.is_some() {
                return a;
            }

            a = match a {
                Some(n) => n.borrow().parent.as_ref().and_then(|w| w.upgrade()),
                None => q.clone(),
            };
            
            b = match b {
                Some(n) => n.borrow().parent.as_ref().and_then(|w| w.upgrade()),
                None => p.clone(),
            };
        }
    }
}