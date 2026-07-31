// Definition for a Node.
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: i32,
//     pub next: Option<Rc<RefCell<Node>>>,
//     pub random: Option<Rc<RefCell<Node>>>,
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         Node {
//             val,
//             next: None,
//             random: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {

    }
}
