use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn get_intersection_node(
        head_a: Option<Rc<RefCell<ListNode>>>,
        head_b: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        let mut p_a = head_a.clone();
        let mut p_b = head_b.clone();

        loop {
            let is_eq = match (&p_a, &p_b) {
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            };

            if is_eq {
                return p_a;
            }

            p_a = match p_a {
                Some(node) => node.borrow().next.clone(),
                None => head_b.clone(),
            };

            p_b = match p_b {
                Some(node) => node.borrow().next.clone(),
                None => head_a.clone(),
            };
        }
    }
}